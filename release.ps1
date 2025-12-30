
git checkout release
git pull
git merge main -m 'chore(release): publish'
git checkout main
git merge checkout
git push