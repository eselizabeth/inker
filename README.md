A static website designed to be simple as possible.

Current commands:

**build**: generates static website based on what is inside the posts folder

**clean**: removes the content of build folder

**new** <postname>: creates a post with given name

example:

`cargo run new how to do x?`

*and the markdown file can be found in posts folder with name as slugged, in this case*: how-to-do-x

**delete** <postname>: deletes the post with given name

**deleteall**: removes all the content inside build and posts folder

Current configuration:

**website-name**: website of the name visible on tabs & website

**posts-per-page**: how many pages should be shown per page, pagination has to be enabled for this one

**pagination**: true or false

**icon-path**: path to the website icon, if it is placed in content/static folder file name would be enough

extra to add custom content, as example given below

**extra**: 
- { $content_path: $template_path, visible-name: $name }
- { projects.md: "projects.html", visible-name: "projects" }

* * *

TODO:
- [x] Generating table of contents based on headers
- [x] Pagination
- [x] Custom page support
- [] Documentation theme
- [x] Live reload