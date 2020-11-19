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
    "assets"
]

all_crates = ["acute_" + crate_name + "/" for crate_name in crate_names]

# these crates are registered but not used yet
new_crates = [
    "audio",
    "physics",
    "physics2d",
    "physics3d",
    "nphysics",
    "particles",
    "network",
    "ron",
    "script",
    "ui",
    "imgui",
    "transform",
    "ai",
    "animation",
    "async",
    "math",
    "mesh",
    "wgpu"
]

all_new_crates = ["acute_" + crate_name + "/" for crate_name in new_crates]
