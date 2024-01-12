<!DOCTYPE html>
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
<input type="hidden" id=commandcount name=commandcount value=0></input>
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
    for ($i = 0; $i < (int)$_POST['commandcount']; $i++) {
        $command =  $_POST['command'.$i];
        switch ($command) {
        case "status":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $location = atoi(substr($status, strpos($status, "songid: ") + strlen("songid: ")));
            fwrite($fp, "playlistid ".$location."\n");
            $playlist = read_to_end($fp);
            $j = strpos($playlist, "file: ") + strlen("file: ");
            while($playlist[$j] != "\n") {
                echo $playlist[$j];
                $j++;
            }
            if ($status[strpos($status, "state: ") + strlen("state: p")] == 'l') {
                echo "<br>[playing] #";
            } else {
                echo "<br>[paused] #";
            }
            echo 1+atoi($playlist, strpos($playlist, "Pos: ") + strlen("Pos: ")) . "/" .
                   atoi($status, strpos($status, "playlistlength: ") + strlen("playlistlength: "));
            $time = atoi($status, strpos($status, "time: ") + strlen("time: "));
            if ($time >= 3600) {
                printf(" %d:%02d:%02d", intdiv($time, 3600), intdiv($time,60)%60, $time%60);
            } else {
                printf(" %d:%02d", intdiv($time,60), $time%60);
            }
            $time = atoi($status, strpos($status, "duration: ") + strlen("duration: "));
            if ($time >= 3600) {
                printf("/%d:%02d:%02d<br>", intdiv($time, 3600), intdiv($time,60)%60, $time%60);
            } else {
                printf("/%d:%02d<br>", intdiv($time,60), $time%60);
            }
            $stat = ["off", "on"];
            printf(
                "volume: %d%% repeat: %s random: %s single: %s consume: %s<br>",
                atoi($status, strpos($status, "volume: ") + strlen("volume: ")),
                $stat[$status[strpos($status, "repeat: ") + strlen("repeat: ")]],
                $stat[$status[strpos($status, "random: ") + strlen("random: ")]],
                $stat[$status[strpos($status, "single: ") + strlen("repeat: ")]],
                $stat[$status[strpos($status, "consume: ") + strlen("consume: ")]]
            );
            break;
        case "toggle":
            fwrite($fp, "pause\n");
            read_to_end($fp);
            break;
        case "discard":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $value = $status[strpos($status, "consume: ") + strlen("consume: ")];

            echo $value;
            if ($value === "0") {
                fwrite($fp, "command_list_begin\n" .
                "consume 1\n" .
                "next\n" .
                "consume 0\n" .
                "command_list_end\n");
            } else {
                fwrite($fp, "next\n");
            }
            read_to_end($fp);
            break;
        case "playlist":
            fwrite($fp, "playlistinfo\n");
            $playlist = explode("file: ", read_to_end($fp));
            $max = count($playlist);
            for ($i = 1; $i < $max; $i++) {
                echo $i . ". " .
                    substr($playlist[$i], 0, strpos($playlist[$i], "\n")) .
                    "<br>";
            }
            break;
        case "repeat":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $value = $status[strpos($status, "repeat: ") + strlen("repeat: ")];
            if ($value === "0") {
                fwrite($fp, "repeat 1\n");
            } else {
                fwrite($fp, "repeat 0\n");
            }
            read_to_end($fp);
            break;
        case "random":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $value = $status[strpos($status, "random: ") + strlen("random: ")];
            if ($value === "0") {
                fwrite($fp, "random 1\n");
            } else {
                fwrite($fp, "random 0\n");
            }
            read_to_end($fp);
            break;
        case "single":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $value = $status[strpos($status, "single: ") + strlen("single: ")];
            if ($value === "0") {
                fwrite($fp, "single 1\n");
            } else {
                fwrite($fp, "single 0\n");
            }
            read_to_end($fp);
            break;
        case "consume":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $value = $status[strpos($status, "consume: ") + strlen("consume: ")];
            if ($value === "0") {
                fwrite($fp, "consume 1\n");
            } else {
                fwrite($fp, "consume 0\n");
            }
            read_to_end($fp);
            break;
        case "update":
            fwrite($fp, "update\n");
            read_to_end($fp);
            break;
        case "volume":
            fwrite($fp, "status\n");
            $status = read_to_end($fp);
            $current = atoi(substr($status, strpos($status, "volume: ") + strlen("volume: ")));
            $sign = $_POST["a".$i][0];
            if ($sign === '-' || $sign === '+') {
                $current += $_POST["a".$i];
            } else {
                $current = $_POST["a".$i];
            }
            $current = min(100, max(0, $current));
            fwrite($fp, "setvol ".$current."\n");
            read_to_end($fp);
            break;
        case "add":
            $input = explode(",", $_POST["a".$i]);
            $max = count($input);
            for ($i = 0; $i < $max; $i++) {
                fwrite($fp, "searchadd \"(file == \\\"".$input[$i]."\\\")\"\n");
                read_to_end($fp);
            }
            break;
        case "remove":
            $input = explode(",", $_POST["a".$i]);
            rsort($input, SORT_NUMERIC);
            $max = count($input);
            for ($i = 0; $i < $max; $i++) {
                echo $input[$i];
                fwrite($fp, "delete " . $input[$i] - 1 . "\n");
                read_to_end($fp);
            }
            break;
        default:
            echo "Invalid command<br>";
            break;
        }
    }
    // fwrite($fp, "status\n");
    // $value = "";
    // while (($val = fgets($fp)) !== "OK\n")  {
    //     $value .= $val;
    // }
    // echo $value;
}

function read_to_end($fp) {
    $value = "";
    while (($val = fgets($fp)) !== "OK\n")  {
        $value .= $val;
    }
    return $value;
}

function atoi($str, $index = 0) {
    $result = 0;
    for ($i = $index; is_numeric($str[$i]); $i++) {
        $result *= 10;
        $result += $str[$i];
    }
    return $result;
}
?>
</body>
</html>
