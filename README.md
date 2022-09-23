A static website designed to be simple as possible. \
Current commands: \
**build**: generates static website based on what is inside the posts folder \
**clean**: removes the content of build folder \
**new** <postname>: creates a post with given name \
example: \
`cargo run new how to do x?` \
*and the markdown file could be accesible from posts folder with name as slugged, in this case: how-to-do-x*
**delete** <postname>: deletes the post with given name \
**deleteall**: removes all the content inside build and posts folder

* * *
TODO:
- [ ] Generating table of contents based on headers
- [ ] Other themes
- [ ] Pagination