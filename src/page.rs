pub const PAGE_TOP: &str =
    "<div class=\"center\">Click the header to sort.</div>
<br />
<table id=\"betters\" class=\"center\">
    <tr>
        <th onclick=\"sortTable2(0)\">Better</th>
        <th onclick=\"sortTable2(1)\">Score</th>
        <th onclick=\"sortTable2(2)\">Wins</th>
        <th onclick=\"sortTable2(3)\">Divisional<br />Losses</th>
    </tr>
";

pub const PAGE_MIDDLE: &str =
    "</table>
<br />
<table id=\"teams\" class=\"center\">
    <tr>
        <th></th>
        <th onclick=\"sortTable(1)\">Team</th>
        <th onclick=\"sortTable(2)\">Score</th>
        <th onclick=\"sortTable(3)\">Wins</th>
        <th onclick=\"sortTable(4)\">Divisional<br />Losses</th>
        <th>Betters</th>
    </tr>
";

pub const PAGE_BOTTOM: &str =
    "</table>
<br />

    
<script>
    function sortTable(n) {
      var table, rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
      table = document.getElementById(\"teams\");
      switching = true;
      // Set the sorting direction to ascending:
      dir = \"asc\";
      /* Make a loop that will continue until
      no switching has been done: */
      while (switching) {
        // Start by saying: no switching is done:
        switching = false;
        rows = table.rows;
        /* Loop through all table rows (except the
        first, which contains table headers): */
        for (i = 1; i < (rows.length - 1); i++) {
          // Start by saying there should be no switching:
          shouldSwitch = false;
          /* Get the two elements you want to compare,
          one from current row and one from the next: */
          x = rows[i].getElementsByTagName(\"TD\")[n];
          y = rows[i + 1].getElementsByTagName(\"TD\")[n];
          /* Check if the two rows should switch place,
          based on the direction, asc or desc: */
          if (dir == \"asc\") {
            if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) {
              // If so, mark as a switch and break the loop:
              shouldSwitch = true;
              break;
            }
          } else if (dir == \"desc\") {
            if (x.innerHTML.toLowerCase() < y.innerHTML.toLowerCase()) {
              // If so, mark as a switch and break the loop:
              shouldSwitch = true;
              break;
            }
          }
        }
        if (shouldSwitch) {
          /* If a switch has been marked, make the switch
          and mark that a switch has been done: */
          rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
          switching = true;
          // Each time a switch is done, increase this count by 1:
          switchcount ++;
        } else {
          /* If no switching has been done AND the direction is \"asc\",
          set the direction to \"desc\" and run the while loop again. */
          if (switchcount == 0 && dir == \"asc\") {
            dir = \"desc\";
            switching = true;
          }
        }
      }
    }

    function sortTable2(n) {
      var table, rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
      table = document.getElementById(\"betters\");
      switching = true;
      // Set the sorting direction to ascending:
      dir = \"asc\";
      /* Make a loop that will continue until
      no switching has been done: */
      while (switching) {
        // Start by saying: no switching is done:
        switching = false;
        rows = table.rows;
        /* Loop through all table rows (except the
        first, which contains table headers): */
        for (i = 1; i < (rows.length - 1); i++) {
          // Start by saying there should be no switching:
          shouldSwitch = false;
          /* Get the two elements you want to compare,
          one from current row and one from the next: */
          x = rows[i].getElementsByTagName(\"TD\")[n];
          y = rows[i + 1].getElementsByTagName(\"TD\")[n];
          /* Check if the two rows should switch place,
          based on the direction, asc or desc: */
          if (dir == \"asc\") {
            if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) {
              // If so, mark as a switch and break the loop:
              shouldSwitch = true;
              break;
            }
          } else if (dir == \"desc\") {
            if (x.innerHTML.toLowerCase() < y.innerHTML.toLowerCase()) {
              // If so, mark as a switch and break the loop:
              shouldSwitch = true;
              break;
            }
          }
        }
        if (shouldSwitch) {
          /* If a switch has been marked, make the switch
          and mark that a switch has been done: */
          rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
          switching = true;
          // Each time a switch is done, increase this count by 1:
          switchcount ++;
        } else {
          /* If no switching has been done AND the direction is \"asc\",
          set the direction to \"desc\" and run the while loop again. */
          if (switchcount == 0 && dir == \"asc\") {
            dir = \"desc\";
            switching = true;
          }
        }
      }
    }
</script>

<style>
@media (max-width: 768px) {
  body, table, th, td {
    font-size: 14px;
  }
}

@media (min-width: 768px) {
  body, table, th, td {
    font-size: 16px;
  }
}

body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
  background-color: #f5f5f5;
}

table {
  border-collapse: collapse;
  background-color: white;
  width: 50%;
  max-width: 1200px;
  margin: 0 auto;
}

th, td {
  padding: 10px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #d25252;
  color: white;
  cursor: pointer;
}

.center {
  text-align: center;
}

.small {
  font-size: 12px;
}

img {
  max-width: 35px;
  height: auto;
}
</style>
";
