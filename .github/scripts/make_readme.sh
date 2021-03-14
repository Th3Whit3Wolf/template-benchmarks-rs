#!/bin/env bash

ech() {
    echo "${1}" >> new_README.md
}

mkRow() {
    local file_path=$1
    local lib=$(basename "${file_path}")
    if [ ! "${lib}" == "report" ]; then
        local perf=$(cat "${file_path}"/report/index.html | grep "<td>Mean</td>" -A 3 | tail -n 3 | cut -d ">" -f2 | cut -d "<" -f1)
        local min=$(echo ${perf} | cut -d ' ' -f1-2 )
        local avg=$(echo ${perf} | cut -d ' ' -f3-4 )
        local max=$(echo ${perf} | cut -d ' ' -f5-6 )
        ech "| ${lib} | ${min} | ${avg} | ${max} |"
    fi
}

mkTable() {
    local title=$1
    shift            # Shift all arguments to the left (original $1 gets lost)
    local arr=("$@") # Rebuild the array with rest of arguments
   
    ech "### ${title}"
    ech
    ech "| Library | Lower bound | Estimate | Upper bound |"
    ech "| ------- | ----------: | -------: | ----------: |"
    for i in "${arr[@]}"; do
        mkRow "$i"
    done
    ech " "
}

main() {
    if cargo build --features make_md && cargo bench; then
    
        shopt -s nullglob
        BigTable=(target/criterion/Big\ table/*)
        Teams=(target/criterion/Teams/*)
        shopt -u nullglob # Turn off nullglob to make sure it doesn't interfere with anything later

        for i in "${BigTable[@]}"; do
            : #echo "$i"
        done

        mkTable "Big Table" "${BigTable[@]}"
        mkTable "Teams" "${Teams[@]}"

        ech "## Running the benchmarks"
        ech
        ech "\`\`\`bash"
        ech "$ cargo bench"
        ech "\`\`\`"
        ech
        ech "Plots will be rendered if \`gnuplot\` is installed and will be available in the \`target/criterion\` folder."
        mv new_README.md README.md
    else
        echo "Failed to update README"
        exit 1
    fi
}

main