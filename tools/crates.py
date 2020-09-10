# in publish order
# 'acute_' will be appended automatically
crate_names = [
    "ecs",
    "core",
    "scenes",
    "window",
    "render_backend",
    "render",
    "input",
    "app",
]

all_crates = ["acute_" + crate_name + "/" for crate_name in crate_names]

