git pull
git add .
git commit -m 'chore(release): publish'
git checkout release
git pull
git merge main
git checkout main
git merge checkout
git push