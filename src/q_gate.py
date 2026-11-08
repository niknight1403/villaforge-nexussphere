import subprocess
import time

def check_environment_health():
    try:
        subprocess.run(['cargo', 'version'], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return True
    except subprocess.CalledProcessError as e:
        if 'spurious network error' in str(e.stderr):
            return False
        else:
            raise e

def retry(func, max_retries=3, backoff_factor=2):
    retries = 0
    while retries < max_retries:
        try:
            return func()
        except Exception as e:
            time.sleep(backoff_factor ** retries)
            retries += 1
    raise Exception('Max retries exceeded')

def q_gate():
    if not retry(check_environment_health):
        raise Exception('Environment health check failed')

if __name__ == '__main__':
    q_gate()