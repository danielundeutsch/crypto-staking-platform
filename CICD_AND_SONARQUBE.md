# 🔄 CI/CD and Code Quality Setup

## 📍 GitHub Actions Location

**Workflows are in**: `/.github/workflows/`

### 1. **Main CI/CD Pipeline** - `ci.yml`
- Runs on push to main/develop branches
- Builds and tests all services
- Creates Docker images
- Runs SonarQube analysis

### 2. **PR Checks** - `pr-checks.yml`
- Runs on pull requests
- Security scanning with Trivy
- License checking
- Commit message validation

## 🔍 SonarQube Configuration

**Config file**: `/sonar-project.properties`

### Features:
- Code quality analysis for Rust and TypeScript
- Test coverage tracking
- Security vulnerability detection
- Code duplication checking

## 🚀 How to Use GitHub Actions

1. **Push to GitHub**:
   ```bash
   git add .
   git commit -m "feat: add new feature"
   git push origin main
   ```

2. **GitHub Actions will automatically**:
   - Run tests for all services
   - Check code formatting (Rust: clippy, TS: ESLint)
   - Build Docker images
   - Run SonarQube analysis
   - Deploy to staging (if on main branch)

## 🛠️ SonarQube Setup

### Option 1: Use SonarCloud (Recommended)
1. Go to https://sonarcloud.io
2. Sign up with GitHub
3. Import your repository
4. Add secrets to GitHub:
   ```
   SONAR_TOKEN=your_token_here
   ```

### Option 2: Self-hosted SonarQube
```bash
# Add to docker-compose-infra.yml
sonarqube:
  image: sonarqube:community
  ports:
    - "9000:9000"
  environment:
    SONAR_ES_BOOTSTRAP_CHECKS_DISABLE: true
  volumes:
    - sonarqube_data:/opt/sonarqube/data
    - sonarqube_logs:/opt/sonarqube/logs
    - sonarqube_extensions:/opt/sonarqube/extensions
```

## 📊 View CI/CD Results

### GitHub Actions:
1. Go to your GitHub repository
2. Click on "Actions" tab
3. View workflow runs

### SonarQube Results:
- **SonarCloud**: https://sonarcloud.io/dashboard?id=crypto-staking-platform
- **Self-hosted**: http://localhost:9000

## 🔧 Local Testing (Before Push)

```bash
# Test Rust services
cd services/api-gateway
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Test Frontend
cd frontend
npm run lint
npm run build
npm test

# Run SonarQube locally
sonar-scanner
```

## 📋 CI/CD Features

✅ **Automated Testing**
- Unit tests for all services
- Integration tests
- Code coverage reports

✅ **Code Quality**
- Rust: cargo fmt, clippy
- TypeScript: ESLint, TypeScript checks
- SonarQube deep analysis

✅ **Security**
- Trivy vulnerability scanning
- Dependency checking
- Secret scanning

✅ **Docker**
- Automated image building
- Push to registry
- Multi-stage builds

✅ **Deployment**
- Automatic staging deployment
- Kubernetes manifests applied
- Health check verification

## 🎯 Current Status

- **GitHub Actions**: ✅ Configured (needs GitHub repository)
- **SonarQube Config**: ✅ Ready (needs token)
- **Local Development**: ✅ Working
- **Remote CI/CD**: ⏳ Awaiting GitHub push

## 📝 Next Steps

1. **Create GitHub Repository**:
   ```bash
   git remote add origin https://github.com/yourusername/crypto-staking-platform.git
   git push -u origin main
   ```

2. **Setup SonarCloud**:
   - Sign up at sonarcloud.io
   - Import repository
   - Get token

3. **Add GitHub Secrets**:
   - Go to Settings → Secrets
   - Add `SONAR_TOKEN`
   - Add Docker registry credentials if needed

4. **First Push**:
   - Make a commit
   - Push to GitHub
   - Watch Actions run!

---

**Note**: GitHub Actions only run when code is pushed to GitHub. They don't run locally.