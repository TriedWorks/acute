import glob
import os
import subprocess
import shutil

# this script converts

# check if not in project folder
if not os.getcwd().endswith("acute"):
    print("not in your project folder")
    raise SystemExit

os.chdir("assets/shaders")

# clear compiled folder
shutil.rmtree("compiled/")

# read all shaders
shaders = glob.glob('*')

# recreate compiled folder
os.mkdir("compiled")

# use glslangvalidator to compile all shaders
for shader in shaders:
    subprocess.run(['glslangvalidator', '-V', shader, '-o', 'compiled/' + shader + '.spv'])
