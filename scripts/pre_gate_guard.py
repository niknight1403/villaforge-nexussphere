import subprocess
import sys
import time

def run_build_with_retry(retries=3, delay=5):
    for i in range(retries):
        print(f"Attempt {i+1}: Running cargo build...")
        result = subprocess.run(["cargo", "build", "--offline"], capture_output=True, text=True)
        if result.returncode == 0:
            print("Build successful.")
            return True
        print(f"Build failed: {result.stderr}")
        time.sleep(delay)
    return False

def verify_environment():
    print("Verifying dependencies and environment...")
    # Check for critical files
    required_files = ["Cargo.toml", "src/crypto_core.rs"]
    for f in required_files:
        try:
            with open(f, 'r') as file:
                pass
        except FileNotFoundError:
            print(f"Error: {f} not found.")
            sys.exit(1)
    
    if not run_build_with_retry():
        print("Critical: Build failed after retries.")
        sys.exit(1)

if __name__ == "__main__":
    verify_environment()