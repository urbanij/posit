from setuptools import setup, find_packages

def build_native(spec):
  # Step 1: build the rust library
  build = spec.add_external_build(cmd=["cargo", "build", "--release"], path=".")

  # Step 2: add a cffi module based on the dylib we built
  #
  # We use lambdas here for dylib and header_filename so that those are
  # only called after the external build finished.
  spec.add_cffi_module(
    module_path="jposit._native",
    dylib=lambda: build.find_dylib("jposit",
                           in_path="target/release"),
    header_filename=lambda: build.find_header("jposit.h",
                                        in_path="."),
    rtld_flags=["NOW", "NODELETE"],
  )


setup(
  name="jposit",
  version="0.1.0",
  packages=find_packages(),
  include_package_data=True,
  zip_safe=False,
  platforms="any",
  install_requires=[
    "milksnake",
  ],
  milksnake_tasks=[
    build_native,
  ],
)
