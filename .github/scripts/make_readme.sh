#!/bin/env bash

ech() {
    echo "${1}" >> new_README.md
}

as_nanosecs() {
    local avg=$(cat "${1}"/report/index.html | grep "<td>Mean</td>" -A 3 | tail -n 3 | cut -d ">" -f2 | cut -d "<" -f1 | head -n2 | tail -n1)
    local num=$(echo ${avg} | cut -d ' ' -f1)
    local units=$(echo ${avg} | cut -d ' ' -f2)
    #echo $avg
    if [ "${units}" == "ms" ]; then
        val=$(printf "%.0f" "$(echo "${num} * 1000000" | bc)")
        echo $val
    elif [ "${units}" == "us" ]; then
        val=$(printf "%.0f" "$(echo "${num} * 1000" | bc)")
        echo $val

    elif [ "${units}" == "ns" ]; then
        val=$(printf "%.0f" "${num}")
        echo $val
    fi   
}

mkRow() {
    local file_path=$1
    local lib=$(basename "${file_path}")
    local perf=$(cat "${file_path}"/report/index.html | grep "<td>Mean</td>" -A 3 | tail -n 3 | cut -d ">" -f2 | cut -d "<" -f1)
    local min=$(echo ${perf} | cut -d ' ' -f1-2 )
    local avg=$(echo ${perf} | cut -d ' ' -f3-4 )
    local max=$(echo ${perf} | cut -d ' ' -f5-6 )
    ech "| ${lib} | ${min} | ${avg} | ${max} |"
}

sort() {
    local arr=("$@")

    for ((i=0; i <= $((${#arr[@]} - 2)); ++i)); do
        for ((j=((i + 1)); j <= ((${#arr[@]} - 1)); ++j)); do
            first_val="$(as_nanosecs ${arr[i]})"
            secnd_val="$(as_nanosecs ${arr[j]})"
            #echo $first_val
            #echo $secnd_val
            if [[ ${first_val} -gt ${secnd_val} ]]; then
                # echo $i $j ${arr[i]} ${arr[j]}
                tmp=${arr[i]}
                arr[i]=${arr[j]}
                arr[j]=$tmp         
            fi
        done
    done
    return "${arr[@]}"
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
        bt=(target/criterion/Big\ table/*)
        tm=(target/criterion/Teams/*)
        shopt -u nullglob # Turn off nullglob to make sure it doesn't interfere with anything later
        BigTable=()
        Teams=()
        for value in "${bt[@]}"; do
            [[ "${value}" != "target/criterion/Big table/report" ]] && BigTable+=("${value}")
        done
        for value in "${tm[@]}"; do
            [[ $value != "target/criterion/Teams/report" ]] && Teams+=($value)
        done
        for ((i=0; i <= $((${#Teams[@]} - 2)); ++i)); do
            for ((j=((i + 1)); j <= ((${#Teams[@]} - 1)); ++j)); do
                first_val=$(as_nanosecs "${Teams[i]}")
                secnd_val=$(as_nanosecs "${Teams[j]}")
                if [[ ${first_val} -gt ${secnd_val} ]]; then
                    tmp=${Teams[i]}
                    Teams[i]=${Teams[j]}
                    Teams[j]=$tmp         
                fi
            done
        done

        for ((i=0; i <= $((${#BigTable[@]} - 2)); ++i)); do
            for ((j=((i + 1)); j <= ((${#BigTable[@]} - 1)); ++j)); do
                first_val=$(as_nanosecs "${BigTable[i]}")
                secnd_val=$(as_nanosecs "${BigTable[j]}")
                if [[ ${first_val} -gt ${secnd_val} ]]; then
                    tmp=${BigTable[i]}
                    BigTable[i]=${BigTable[j]}
                    BigTable[j]=$tmp         
                fi
            done
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
        ech
        ech "##### Last ran on $(date +"%d %B %Y")"
        mv new_README.md README.md
    else
        echo "Failed to update README"
        exit 1
    fi
}

main
