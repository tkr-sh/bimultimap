right:
    echo '//! This is a generated file! Don'"'"'t modify it!!!' > src/right.rs
    cat src/left.rs >> src/right.rs

    sed -i 's/Left/__Temporary/g' src/right.rs
    sed -i 's/Right/Left/g' src/right.rs
    sed -i 's/__Temporary/Right/g' src/right.rs


    sed -i 's/left/__temporary/g' src/right.rs
    sed -i 's/right/left/g' src/right.rs
    sed -i 's/__temporary/right/g' src/right.rs



test:
    cargo nextest run

run:
    cargo run
