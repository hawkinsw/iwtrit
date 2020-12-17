TEMPFILE := $(shell mktemp)
csv: src/main.rs csv
	cargo build --release;
	echo "i, i, duration_iterator, duration_while" > ${TEMPFILE}
	for i in `seq 1 1000`; do ./target/release/episode2; done >>${TEMPFILE};
	sed -i 's/Ok(//g' ${TEMPFILE};
	sed -i 's/ns)//g' ${TEMPFILE};
	mv ${TEMPFILE} ./csv.csv;
	rm -f ${TEMPFILE};

clean:
	rm -f csv.csv;
	cargo clean
