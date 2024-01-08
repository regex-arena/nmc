<!doctype html>
<html>
<!-- Input form -->

<!-- Run commands -->
<?php
$fp = fsockopen("127.0.0.1", 6600);
fwrite($fp, "pause\n");
echo fgets($fp);
?>
</html>