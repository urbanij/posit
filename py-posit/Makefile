export CARGO_TARGET_DIR=target

all: build install
.PHONY: pre-build build install

pre-build:
	mkdir -p target

build: pre-build
	python setup.py bdist_wheel
	# python -m build

install:
	pip install --force-reinstall dist/jposit-*py3-none-*.whl

demo:
	cd && python -c "import jposit; print(f'jposit v:{jposit.__version__}'); jposit.demo()"

clean:
	rm -rf build dist *.egg-info
	rm -rf jposit/*.so 
