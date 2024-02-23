const pk = require("./pkg/wasm_table_project.js");
const fs = require("node:fs");


// Now you can use the add function
const result = pk.gettingstring("checking....");
console.log(result); 


 
 try {
    const jsonDatapath = fs.readFileSync('./utils/table_task_hashmap_json/Data.json', 'utf8');
    const jsonData = JSON.stringify(JSON.parse(jsonDatapath));
    
    const hashmapTableJson = pk.process_json(jsonData);
    // Write hashmap table JSON to a file
    fs.writeFileSync('./utils/table_task_hashmap_json/hashmap_table_data.json', hashmapTableJson);

    console.log('Data processed successfully.');
} catch (error) {
    console.error('Error:', error.message);
}