## the following command searches for lines in a poem.txt and outputs them to output. You can run in pwsh

cargo run -- to poem.txt > output.txt

## In powershell you can set environmental variable, if yo uwant to perform case insensitive search

$Env:IGNORE_CASE=1; cargo run -- to poem.txt    