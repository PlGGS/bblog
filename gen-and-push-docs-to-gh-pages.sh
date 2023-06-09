git checkout master
cargo doc
git pull origin master
git checkout gh-pages
git merge master --no-commit
git checkout master -- target/doc
git commit -m "Updated docs"
git push origin gh-pages