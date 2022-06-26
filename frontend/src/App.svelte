<script>
    export let bindings;
    import {Content, FileUploader} from "carbon-components-svelte";

    let {parse} = bindings;
    let files = [];
    let content ="";
    let reader = new FileReader();
    reader.addEventListener("load", () => {
        content = reader.result;
        let parsed = parse(content);
        console.log(parsed);
    }, false);

    let add_handler = (e) => {
        reader.readAsText(e.detail[0]);
    };

</script>

<Content>
    <h1>Rust: </h1>
    <div>
        <FileUploader
                labelTitel="Upload a file"
                buttonLabel="Add file"
                labelDescription="Only txt files are accepted"
                accept={[".txt"]}
                bind:files
                status="complete"
                on:add={add_handler}
        />
    </div>
    <h3>
        File contents are: {content}
    </h3>
</Content>


<style>
    h1, h3 {
        text-align: center;
        color: #ff3e00;
        text-transform: uppercase;
        font-weight: 100;
    }

    div {
        margin: 10px 0;
    }
</style>