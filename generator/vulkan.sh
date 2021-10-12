echo "#![allow(non_upper_case_globals)]" > artifact.rs
echo "#![allow(non_camel_case_types)]" >> artifact.rs
echo "#![allow(non_snake_case)]" >> artifact.rs
cat - >> artifact.rs
cat artifact.rs | awk -f fix_pointer.awk > vulkan_generated.rs
rm artifact.rs
