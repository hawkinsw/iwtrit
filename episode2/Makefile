TEMPFILE := $(shell mktemp)
csv: src/main.rs csv
	cargo build --release;
	echo "duration_iterator, duration_while" > ${TEMPFILE}
	for i in `seq 1 25`; do ./target/release/episode2; done >>${TEMPFILE};
	sed -i 's/Ok(//g' ${TEMPFILE};
	sed -i 's/[a-zA-Z]\+)//g' ${TEMPFILE};
	mv ${TEMPFILE} ./csv.csv;
	rm -f ${TEMPFILE};

clean:
	rm -f csv.csv;
	cargo clean
