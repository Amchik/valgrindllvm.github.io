from glob import glob
from subprocess import run
from json import loads
from collections import defaultdict

pattern = 'src/*.fmf'
template = "template.html"
template_out = "out/template.html"
posts_out = "src/posts.fmf"
fmfcc = "target/release/fmfcc"

posts = dict()

for file in glob(pattern):
    res = run([fmfcc, "query", "--json", file], capture_output=True)
    if res.returncode != 0:
        print(f"Failed to query file '{file}': {res.stderr}")
        exit(1)
    meta = loads(res.stdout)
    if not meta["author"]: continue
    posts[file] = meta

tmp = ""
with open(template, "r") as f:
    tmp = f.read()

posts_template = ""
for file, meta in posts.items():
    posts_template += f'<a href="{file[3:-4]}" class="post-card">'
    posts_template += f'<span class="title">{meta["title"]}</span>'
    desc = f'{meta["author"]} • {meta["date"]} • {meta["category"]}'
    posts_template += f'<span class="description">{desc}</span>'
    posts_template += '</a>'

tmp = tmp.replace("{posts}", posts_template)

css = ""
for style in glob('styles/*.css'):
    with open(style, "r") as f:
        css += f.read() + "\n"
tmp = tmp.replace("{styles}", css)

with open(template_out, "w") as f:
    f.write(tmp)

posts_fmf = """
# List of public posts (by category)

"""

posts_by_category = defaultdict(list)
for file, meta in posts.items():
    category = meta["category"]
    meta["file"] = file
    posts_by_category[category].append(meta)

for category, posts in posts_by_category.items():
    posts_fmf += f"\n## {category}\n\n"
    for meta in posts:
        posts_fmf += f'- \\({meta["file"][3:-4]})[{meta["title"]}] by {meta["author"]}\n'

with open(posts_out, "w") as f:
    f.write(posts_fmf)

