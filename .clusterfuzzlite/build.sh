
cd $SRC/knyst


cargo fuzz build --strip-dead-code -O --debug-assertions "$@"
FUZZ_TARGET_OUTPUT_DIR=target/x86_64-unknown-linux-gnu/release

for f in fuzz/fuzz_targets/*.rs
do
    FUZZ_TARGET_NAME=$(basename ${f%.*})
    # Copy each compiled fuzz target to the output folder
    # the clusterfuzz lite backkend will execute each binary in the folder as a
    # fuzz target
    cp $FUZZ_TARGET_OUTPUT_DIR/$FUZZ_TARGET_NAME $OUT/
done

