#!/usr/bin/env python3

import os
import pathlib
import subprocess


def main():
    os.chdir(pathlib.Path(os.path.realpath(__file__)).parent.parent)

    _check_general_cleanliness()
    _test_rust()


def _check_general_cleanliness():
    working_folder = pathlib.Path.cwd()
    subprocess.run(
        [
            "podman",
            "run",
            "--entrypoint",
            "sh",
            "--rm",
            "--volume",
            f"{working_folder}:/workdir",
            "evolutics/travel-kit:0.8.0",
            "-c",
            "git ls-files -z | xargs -0 travel-kit check --",
        ],
        check=True,
    )


def _test_rust():
    subprocess.run(["rustup", "component", "add", "rustfmt"], check=True)
    subprocess.run(["cargo", "fmt", "--all", "--", "--check"], check=True)

    subprocess.run(["rustup", "component", "add", "clippy"], check=True)
    subprocess.run(
        [
            "cargo",
            "clippy",
            "--all-features",
            "--all-targets",
            "--",
            "--allow",
            "clippy::needless_doctest_main",
            "--deny",
            "warnings",
        ],
        check=True,
    )

    subprocess.run(["cargo", "check"], check=True)
    subprocess.run(["cargo", "test"], check=True)

    examples = [path.stem for path in pathlib.Path("examples").glob("*.rs")]
    for example in sorted(examples):
        _run_example(example, [])
        _run_example(example, ["--release"])


def _run_example(name, extra_arguments):
    subprocess.run(
        ["cargo", "build", "--example", name] + extra_arguments,
        check=True,
    )

    try:
        subprocess.run(
            ["cargo", "run", "--example", name] + extra_arguments,
            check=True,
            timeout=_EXAMPLE_TIMEOUT_IN_SECONDS.get(name),
        )
    except subprocess.TimeoutExpired:
        pass


_EXAMPLE_TIMEOUT_IN_SECONDS = {
    "library_actix_web": 2,
    "library_rocket": 2,
    "library_tide": 2,
    "library_warp": 2,
    "showcase": 2,
}

if __name__ == "__main__":
    main()
