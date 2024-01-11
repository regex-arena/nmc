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
    
    container.appendChild(document.createTextNode("Command: "));
    var selector = container.appendChild(document.createElement("select"));
    selector.setAttribute("name", "command"+index);
    selector.setAttribute("onChange", "command_update("+index+")");
    selector.setAttribute("id", "b"+index);
    var commands = ["status", "toggle", "discard", "playlist", "repeat", "random", "single", "consume", "update", "volume", "add", "remove"];
    var commandscount = commands.length;
    for (var i = 0; i < commandscount; i++) {
        var option = selector.appendChild(document.createElement("option"));
        option.setAttribute("value", commands[i]);
        option.appendChild(document.createTextNode(commands[i]));
    }

    var input = container.appendChild(document.createElement("input"));
    input.setAttribute("value", "");
    input.setAttribute("type", "hidden");
    input.setAttribute("id", "a"+index);


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
    
    // Remove command at index 'index'
    document.getElementById(index).remove();
    
    // Decrement commands with indecies greater then 'index'
    for (var i = index + 1; i < maxindex; i++) {
        document.getElementById("0" + i).setAttribute("id", "0" + (i-1));
        document.getElementById(i).setAttribute("id", i-1);
    }
}

function command_update(index) {
    var dropdown = document.getElementById("b"+index);
    var text_input = document.getElementById("a"+index);
    // Commands that need text input
    var text_commands = ["add", "remove"];
    if (text_commands.includes(dropdown.value)) {
        text_input.setAttribute("type", "text");
        text_input.setAttribute("required", "true");
    } else {
        text_input.setAttribute("type", "hidden");
        text_input.setAttribute("required", "false");
    }
}
