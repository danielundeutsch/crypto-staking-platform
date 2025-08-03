# 🚀 Push to GitHub Instructions

## Step 1: Create GitHub Repository

1. Go to: https://github.com/new
2. Repository name: `crypto-staking-platform`
3. Description: "Microservices platform for crypto token staking"
4. Choose Public or Private
5. **DON'T** initialize with README, .gitignore, or license
6. Click "Create repository"

## Step 2: Connect and Push

After creating the repository, run these commands:

```bash
# Add your GitHub repository as remote origin
# Replace YOUR_USERNAME with your GitHub username
git remote add origin https://github.com/YOUR_USERNAME/crypto-staking-platform.git

# Or if using SSH:
# git remote add origin git@github.com:YOUR_USERNAME/crypto-staking-platform.git

# Push to GitHub
git push -u origin main
```

## Step 3: Add Secrets for CI/CD

After pushing, go to your repository settings:

1. Go to Settings → Secrets and variables → Actions
2. Add these secrets:
   - `SONAR_TOKEN` - Get from https://sonarcloud.io after importing your repo
   - `DOCKER_USERNAME` - If using Docker Hub
   - `DOCKER_PASSWORD` - If using Docker Hub

## Step 4: Enable GitHub Actions

1. Go to Actions tab in your repository
2. You should see workflows ready to run
3. They will run automatically on next push

## Example Commands (replace YOUR_USERNAME):

```bash
# For HTTPS:
git remote add origin https://github.com/YOUR_USERNAME/crypto-staking-platform.git
git push -u origin main

# For SSH (if you have SSH keys setup):
git remote add origin git@github.com:YOUR_USERNAME/crypto-staking-platform.git
git push -u origin main
```

## Verify Push Success

After pushing, you should see:
- All code in GitHub repository
- GitHub Actions starting to run
- Green checkmarks when CI/CD passes

## 🎉 Success Checklist

- [ ] Created GitHub repository
- [ ] Added remote origin
- [ ] Pushed code to GitHub
- [ ] GitHub Actions running
- [ ] Added SONAR_TOKEN secret
- [ ] CI/CD pipeline working

## Troubleshooting

If you get authentication errors:
1. Make sure you're logged into GitHub
2. For HTTPS: Use Personal Access Token instead of password
3. For SSH: Make sure your SSH key is added to GitHub

To create Personal Access Token:
1. Go to GitHub → Settings → Developer settings
2. Personal access tokens → Generate new token
3. Give it repo permissions
4. Use token as password when pushing