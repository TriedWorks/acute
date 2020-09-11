# config:
import os
import shutil
import subprocess
import toml

reserve_only = True

new_crate_names = [
    "audio",
]

print(os.getcwd())

new_crates = ["acute_" + crate_name for crate_name in new_crate_names]

# check if not in project folder
if not os.getcwd().endswith("acute"):
    print("not in your project folder")
    raise SystemExit

# creates 'acute/tools/tmp/<crate_name>' and publishes it
# deletes directory 'acute/tools/tmp' afterwards
if reserve_only:
    if os.getcwd().endswith("acute"):
        os.chdir("tools/")
        os.mkdir("tmp/")
        os.chdir("tmp/")
        print(os.getcwd())
        for crate in new_crates:
            # create new cargo lib and enter it
            subprocess.run(['cargo', 'new', crate, '--lib'])
            os.chdir(crate + "/")

            # Add relevant information to Cargo.toml
            crate_name = crate.replace("acute_", "")  # extract name
            cargo_toml_file = open("Cargo.toml", "r")  # open toml file
            cargo_toml_data = open("Cargo.toml", "r").read()   # read toml file
            cargo_toml_file.close()  # close file
            toml_data = toml.loads(cargo_toml_data)  # convert file data string to dictionary
            toml_data['package']['description'] = crate_name + ' sub-crate of acute'  # add description
            toml_data['package']['license'] = 'Apache-2.0'  # add license
            toml_data['package']['repository'] = "https://github.com/tryworks/acute"  # add git repository
            cargo_toml_file = open("Cargo.toml", "w")
            cargo_toml_file.write(toml.dumps(toml_data))
            cargo_toml_file.close()

            # Add Readme.md
            read_me = open("Readme.md", "w+")
            read_me.write('# A name reserve for the acute project \n'
                          'if you need this crate name, contact contact@strobl.net')
            read_me.close()

            # publish
            subprocess.run(['cargo', 'publish', '--allow-dirty'])
            os.chdir("../")

        os.chdir("../")
        print(os.getcwd())
        shutil.rmtree("tmp/")
