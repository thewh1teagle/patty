#!/bin/sh
# {template_description}
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"{template_bin_path}":*)
        ;;
    *)
        # Prepending path in case a system-installed rustc needs to be overridden
        export PATH="{template_bin_path}:$PATH"
        ;;
esac