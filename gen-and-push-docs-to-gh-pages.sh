git checkout main
git pull origin main
cargo doc
cp -r target/doc ..
rm -rf target/doc
git checkout gh-pages
cp -r ../doc target/doc
rm -rf ../doc
git add .
git commit -m "Updated docs"
git push origin gh-pages
git checkout main
