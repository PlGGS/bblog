git checkout main
git pull origin main
cargo doc
git checkout gh-pages
git merge main --no-commit
git checkout main -- target/doc
git commit -m "Updated docs"
git push origin gh-pages
git checkout main
rm -rf target
