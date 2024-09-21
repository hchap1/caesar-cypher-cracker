# caesar-cypher-cracker
An &lt;overcomplicated> project to efficiently break caesar cyphers written in rust!
<br/>
Usage:<br/>
- caesar_cracker&lt;.exe> "some caesar cypher"
- caesar_cracker&lt;.exe> "some filepath" --load_file
<br/>
Flags:<br/>
--animate -> Outputs each possibility as it is considered, adds short delay so you can see each print.<br/>
--load_file -> Rather than specifying the cypher inline, load it from a file.<br/>
--save_file -> Rather than outputting result with stdout, write to output.txt.<br/>
--full_check -> Check every word for dictionary validity rather than a portion.<br/>
^^^^^^^^^^^^ causes significant performance loss for longer strings.
