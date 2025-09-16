pub const PAGE_TOP: &str =
    "
<div class=\"center\">Click the header to sort.<br />Click a name in the betters table to filter teams.</div>
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
<div class=\"center\">
    <button onclick=\"showAllTeams()\" id=\"clearFilter\">Show All Teams</button>
    <span id=\"filterStatus\"></span>
</div>
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

    function filterByBetter(betterName) {
      var table = document.getElementById(\"teams\");
      var rows = table.rows;
      var visibleCount = 0;
      
      // Loop through all team rows (skip header row at index 0)
      for (var i = 1; i < rows.length; i++) {
        var bettersColumn = rows[i].getElementsByTagName(\"TD\")[5]; // Betters column is index 5
        var bettersText = bettersColumn.innerHTML.toLowerCase();
        var searchName = betterName.toLowerCase();
        
        // Check if the better name appears in the betters column
        if (bettersText.includes(searchName)) {
          rows[i].style.display = \"\";
          visibleCount++;
        } else {
          rows[i].style.display = \"none\";
        }
      }
      
      // Update filter status
      var filterStatus = document.getElementById(\"filterStatus\");
      filterStatus.innerHTML = \"Filtered by: <strong>\" + betterName + \"</strong> (\" + visibleCount + \" teams)\";
      
      // Remove previous highlighting
      var clickableNames = document.getElementsByClassName(\"clickable-name\");
      for (var i = 0; i < clickableNames.length; i++) {
        clickableNames[i].classList.remove(\"active-filter\");
      }
      
      // Highlight the clicked name
      var clickedElements = document.querySelectorAll('[onclick=\"filterByBetter(\\'' + betterName + '\\')\"]');
      if (clickedElements.length > 0) {
        clickedElements[0].classList.add(\"active-filter\");
      }
    }
    
    function showAllTeams() {
      var table = document.getElementById(\"teams\");
      var rows = table.rows;
      
      // Show all rows
      for (var i = 1; i < rows.length; i++) {
        rows[i].style.display = \"\";
      }
      
      // Clear filter status
      var filterStatus = document.getElementById(\"filterStatus\");
      filterStatus.innerHTML = \"\";
      
      // Remove highlighting from all names
      var clickableNames = document.getElementsByClassName(\"clickable-name\");
      for (var i = 0; i < clickableNames.length; i++) {
        clickableNames[i].classList.remove(\"active-filter\");
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

.clickable-name {
  cursor: pointer;
  color: #0066cc;
  font-weight: bold;
}

.clickable-name:hover {
  background-color: #f0f8ff;
  text-decoration: underline;
}

.active-filter {
  background-color: #d25252 !important;
  color: white !important;
}

#clearFilter {
  background-color: #d25252;
  color: white;
  border: none;
  padding: 8px 16px;
  cursor: pointer;
  border-radius: 4px;
  margin-right: 10px;
}

#clearFilter:hover {
  background-color: #b04040;
}

#filterStatus {
  color: #666;
  font-style: italic;
}
</style>
";
