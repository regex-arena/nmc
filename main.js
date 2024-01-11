function add_command() {
    // Get current command index
    var index = Number(document.getElementById("commandcount").value);
    // Increment command index
    document.getElementById("commandcount").setAttribute("value", 1 + index);
    
    // Get parent element
    var parent = document.getElementById("commandlist");

    // Create new command window
    var container = parent.appendChild(document.createElement("div"));
    container.setAttribute("class", "command");
    container.setAttribute("id", index);
    
    var selector = container.appendChild(document.createElement("input"));
    selector.setAttribute("value", "");
    var input = container.appendChild(document.createElement("input"));


    var remove = container.appendChild(document.createElement("button"));
    remove.setAttribute("type", "button");
    remove.setAttribute("onclick", "remove_command("+index+");");
    // Allows for the element to be found when re-indexing after removal
    remove.setAttribute("id", "0" + index);
    remove.textContent = "-";
}

function remove_command(index) {
    // Decrement command index
    document.getElementById("commandcount").setAttribute("value", maxindex - 1);

    // Get current command index
    var maxindex = Number(document.getElementById("commandcount").value);
    console.log(maxindex);
    
    // Remove command at index 'index'
    document.getElementById(index).remove();
    
    // Decrement commands with indecies greater then 'index'
    for (var i = index + 1; i < maxindex; i++) {
        document.getElementById("0" + i).setAttribute("id", "0" + (i-1));
        document.getElementById(i).setAttribute("id", i-1);
    }
}
