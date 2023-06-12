git checkout main
cargo doc
git pull origin main
git checkout gh-pages
git merge main --no-commit
git checkout main -- target/doc
git commit -m "Updated docs"
git push origin gh-pages