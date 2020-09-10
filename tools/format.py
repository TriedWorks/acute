import subprocess
import os
from crates import all_crates

# check if not in project folder
if not os.getcwd().endswith("acute"):
    print("not in your project folder")
    raise SystemExit

# format acute/
subprocess.run(['cargo', 'fmt'])

# go into crates directory
os.chdir("crates/")

# go into each crate's directory, clean it and go back to 'crates/'
for crate in all_crates:
    os.chdir(crate + "/")
    subprocess.run(['cargo', 'fmt'])
    os.chdir("../")
