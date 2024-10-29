CARGO = cargo
VENV_DIR = venv
PYTHON = $(VENV_DIR)/bin/python
TARGET = target

.PHONY: venv
venv:
	python -m venv $(VENV_DIR)
	${VENV_DIR}/bin/pip install -r requirements.txt

.PHONY: run
run: venv
	$(CARGO) run && $(PYTHON) src/main.py

.PHONY: clean
clean:
	rm -rf $(VENV_DIR) ${TARGET}
	rm data/coordinates.txt data/spiderweb.svg