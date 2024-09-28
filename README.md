static site generator using tera template engine

directory structure
```yaml
posts/          # where the posts will be written in markdown
content/        # static content for template usage
build/          # where the webserver generated website for internal usage
publish/        # where the final output of the static website will be generated
templates/      # template folder
config.yaml     # configuration file
./inker:        # executable
```

**current commands**

**new** postname: creates a post with given name inside the posts folder

**server**: opens a webserver to display current website, the changes on the posts or templates will cause a reload on the browser

**clean**: removes the content of build folder

**delete** postname: deletes the post with given name

**deleteall**: removes all of the content inside build and posts folder

---

**current configuration**

**base-url**: absolute url of the website (eg. https://mark.github.io/my-website/) 

**port**: port of the webserver

**website-name**: website of the name visible on tabs & website

**template-name**: name of the template folder (will be searched inside the templates folder)

**posts-per-page**: how many pages should be shown per page, pagination has to be enabled for this one

**pagination**: true or false to enable/disable pagination

**icon-path**: path to the website icon, should be placed inside the content/static folder

**extra**: extra can be used to add a custom content, giving the respective markdown & template file, example below 
- { $content_path: $template_path, visible-name: $name }
- { projects.md: "projects.html", visible-name: "projects" }

* * *

TODO:
- [x] Generating table of contents based on headers
- [x] Pagination
- [x] Custom page support
- [x] Documentation theme
- [x] Live reload
- [x] Custom model data