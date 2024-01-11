<!doctype html>
<html>
<head>
<link rel="stylesheet" href="style.css">
<script src="main.js"></script>
<title>NMC</title>
</head>
<body>
<!-- Input form -->
<h1>NMC</h1>
<form method=POST>
<div class="location">
Control webpage for MPD server on
<input id=ip value=<?php
if (isset($_POST['ip'])) {
    echo $_POST['ip'];
} else {
    echo '127.0.0.1';
}
?> name=ip></input>:<input type=number id=port value=<?php
if (isset($_POST['port'])) {
    echo $_POST['port'];
} else {
    echo '6600';
}
?> id=port name=port></input>
</div>
<input type="hidden" id=commandcount value=0></input>
<h3>Commands</h3> <button type=button onclick="add_command();">＋</button>
<div id="commandlist">
</div>
<input type="submit" value="Execute">
</form>
<!-- Run commands -->
<?php
if (isset($_POST)) {
    // IMPORTANT:
    // Verify ip and port
    $fp = fsockopen($_POST['ip'], $_POST['port']);
    fwrite($fp, "status\n");
    $value = "";
    while (($val = fgets($fp)) !== "OK\n")  {
        $value .= $val;
    }
    echo $value;
}
?>
</body>
</html>
