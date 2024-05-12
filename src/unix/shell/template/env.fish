# {template_description}
if not contains "{template_bin_path}" $PATH
    # Prepending path in case a system-installed rustc needs to be overridden
    set -x PATH "{template_bin_path}" $PATH
end