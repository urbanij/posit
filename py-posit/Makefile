all: build install
.PHONY: pre-build build install

# create venv and activate it before this: uv venv; source .venv/bin/activate

install-deps:
	uv pip install maturin pip

build: install-deps
	maturin develop
	# maturin build

# install:
# 	pip install --force-reinstall dist/jposit-*py3-none-*.whl

demo:
	python -c "import sys, jposit; print(f'Python{sys.version}'); print(f'jposit v:{jposit.__version__}'); jposit.demo()"

clean:
	rm -rf */**/*.so */**/__pycache__
