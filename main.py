from pathlib import Path
import os

def main():
    for f in os.listdir('/tmp/'):
        path = Path(f'/tmp/{f}')
        try:
            if path.is_file:
                print("File removed successfully")
                os.remove(path)
        except Exception as ex:
            print(f"An error occurred while checking the path. {ex}")

if __name__ == '__main__':
    print("Would you like to clean your system? (Yes/no)")
    userinput = input()
    if userinput.lower() == 'yes':
        main()
