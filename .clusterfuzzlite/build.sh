
cd $SRC/knyst

find / -name alsa 2>/dev/null
find / -name "libasound*" 2>/dev/null

PATH="/usr/include:/usr/lib/x86_64-linux-gnu:$PATH" cargo fuzz build --strip-dead-code -O --debug-assertions "$@"
FUZZ_TARGET_OUTPUT_DIR=target/x86_64-unknown-linux-gnu/release

cp -r --parents /usr/lib/x86_64-linux-gnu/libasound* $OUT/
cp -r --parents /usr/include/alsa $OUT/

for f in fuzz/fuzz_targets/*.rs
do
    FUZZ_TARGET_NAME="random_target" # copy only one for now$(basename ${f%.*})
    # Copy each compiled fuzz target to the output folder
    # the clusterfuzz lite backkend will execute each binary in the folder as a
    # fuzz target
    cp $FUZZ_TARGET_OUTPUT_DIR/$FUZZ_TARGET_NAME $OUT/
    break
done

