pub const PAGE_HEADER: &str =
    "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>CRGA NFL Pool Tracker</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #52576e 0%, #4b4253 100%);
            min-height: 100vh;
            padding: 20px;
        }

        .container {
            max-width: 1400px;
            margin: 0 auto;
        }

        .header {
            text-align: center;
            color: white;
            margin-bottom: 30px;
            animation: fadeIn 0.6s ease-out;
        }

        .header h1 {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 10px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.2);
        }

        .updated-time {
            background: rgba(255,255,255,0.2);
            backdrop-filter: blur(10px);
            padding: 8px 20px;
            border-radius: 20px;
            display: inline-block;
            font-size: 0.9rem;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 30px;
            animation: slideUp 0.8s ease-out;
        }

        .stat-card {
            background: white;
            border-radius: 15px;
            padding: 20px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            transition: transform 0.3s ease, box-shadow 0.3s ease;
        }

        .stat-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 40px rgba(0,0,0,0.3);
        }

        .stat-label {
            font-size: 0.85rem;
            color: #666;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 5px;
        }

        .stat-value {
            font-size: 2rem;
            font-weight: 700;
            color: #667eea;
        }

        .card {
            background: white;
            border-radius: 20px;
            padding: 25px;
            margin-bottom: 25px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            animation: slideUp 1s ease-out;
        }

        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
            flex-wrap: wrap;
            gap: 15px;
        }

        .card-title {
            font-size: 1.5rem;
            font-weight: 700;
            color: #333;
        }

        .controls {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
        }

        .btn {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 25px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.3s ease;
            font-size: 0.9rem;
        }

        .btn:hover {
            transform: scale(1.05);
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
        }

        .btn-secondary {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        }

        .search-box {
            padding: 10px 20px;
            border: 2px solid #e0e0e0;
            border-radius: 25px;
            outline: none;
            transition: all 0.3s ease;
            font-size: 0.9rem;
        }

        .search-box:focus {
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }

        table {
            width: 100%;
            border-collapse: separate;
            border-spacing: 0;
        }

        th {
            background: #667eea;
            color: white;
            padding: 15px;
            text-align: left;
            font-weight: 600;
            cursor: pointer;
            transition: background 0.3s ease;
            position: sticky;
            top: 0;
            z-index: 10;
        }

        th:first-child {
            border-top-left-radius: 10px;
        }

        th:last-child {
            border-top-right-radius: 10px;
        }

        th:hover {
            background: linear-gradient(135deg, #5568d3 0%, #65408b 100%);
        }

        td {
            padding: 15px;
            border-bottom: 1px solid #f0f0f0;
            transition: background 0.2s ease;
        }

        tr:hover td {
            background: #f8f9ff;
        }

        .clickable-name {
            color: #667eea;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s ease;
        }

        .clickable-name:hover {
            color: #764ba2;
            text-decoration: underline;
        }

        .active-filter {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white !important;
            padding: 5px 10px;
            border-radius: 5px;
        }

        .badge {
            display: inline-block;
            padding: 4px 10px;
            border-radius: 12px;
            font-size: 0.85rem;
            font-weight: 600;
            margin: 2px;
        }

        .badge-success {
            background: #d4edda;
            color: #155724;
        }

        .badge-danger {
            background: #f8d7da;
            color: #721c24;
        }

        .badge-warning {
            background: #fff3cd;
            color: #856404;
        }

        img {
            max-width: 35px;
            height: auto;
            transition: transform 0.2s ease;
        }

        img:hover {
            transform: scale(1.2);
        }

        .filter-status {
            background: rgba(102, 126, 234, 0.1);
            color: #667eea;
            padding: 10px 20px;
            border-radius: 10px;
            font-weight: 600;
            display: inline-block;
            margin-top: 10px;
        }

        .small {
            font-size: 0.85rem;
            color: #666;
        }

        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(-20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        @keyframes slideUp {
            from {
                opacity: 0;
                transform: translateY(30px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        @media (max-width: 768px) {
            .header h1 {
                font-size: 1.8rem;
            }
            
            .card {
                padding: 15px;
            }
            
            th, td {
                padding: 10px 8px;
                font-size: 0.85rem;
            }
            
            .stats-grid {
                grid-template-columns: 1fr 1fr;
            }
        }

        .view-toggle {
            display: flex;
            background: #f0f0f0;
            border-radius: 25px;
            padding: 3px;
        }

        .view-btn {
            padding: 8px 16px;
            border: none;
            background: transparent;
            border-radius: 22px;
            cursor: pointer;
            transition: all 0.3s ease;
            font-weight: 600;
        }

        .view-btn.active {
            background: white;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
    </style>
</head>
<body>
    <div class=\"container\">
        <div class=\"header\">
            <h1>🏈 CRGA NFL Pool Tracker</h1>        
";

pub const PAGE_TOP: &str =
    "</div>
        <div class=\"card\">
            <div class=\"card-header\">
                <h2 class=\"card-title\">Leaderboard</h2>
                <div class=\"controls\">
                    <input type=\"text\" class=\"search-box\" placeholder=\"Search betters...\" id=\"searchBetters\">
                </div>
            </div>
            <table id=\"betters\">
                <thead>
                    <tr>
                        <th onclick=\"sortTable2(0)\">Better</th>
                        <th onclick=\"sortTable2(1)\">Score</th>
                        <th onclick=\"sortTable2(2)\">Wins</th>
                        <th onclick=\"sortTable2(3)\">Divisional Losses</th>
                    </tr>
                </thead>
                <tbody>
";

pub const PAGE_MIDDLE: &str =
    "
                </tbody>
            </table>
        </div>

        <div class=\"card\">
            <div class=\"card-header\">
                <h2 class=\"card-title\">Teams</h2>
                <div class=\"controls\">
                    <button onclick=\"showAllTeams()\" class=\"btn\">Show All Teams</button>
                    <input type=\"text\" class=\"search-box\" placeholder=\"Search teams...\" id=\"searchTeams\">
                </div>
            </div>
            <div id=\"filterStatus\" class=\"filter-status\" style=\"display: none;\"></div>
            <table id=\"teams\">
                <thead>
                    <tr>
                        <th></th>
                        <th onclick=\"sortTable(1)\">Team</th>
                        <th onclick=\"sortTable(2)\">Score</th>
                        <th onclick=\"sortTable(3)\">Wins</th>
                        <th onclick=\"sortTable(4)\">Divisional Losses</th>
                        <th>Betters</th>
                    </tr>
                </thead>
                <tbody>
";

pub const PAGE_BOTTOM: &str =
    "
              </tbody>
            </table>
        </div>
    </div>

    <script>
        function sortTable(n) {
            var table = document.getElementById(\"teams\");
            var rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
            switching = true;
            dir = \"asc\";
            
            while (switching) {
                switching = false;
                rows = table.rows;
                
                for (i = 1; i < (rows.length - 1); i++) {
                    shouldSwitch = false;
                    x = rows[i].getElementsByTagName(\"TD\")[n];
                    y = rows[i + 1].getElementsByTagName(\"TD\")[n];
                    
                    var xContent = n === 2 || n === 3 || n === 4 ? parseFloat(x.innerHTML) : x.innerHTML.toLowerCase();
                    var yContent = n === 2 || n === 3 || n === 4 ? parseFloat(y.innerHTML) : y.innerHTML.toLowerCase();
                    
                    if (dir == \"asc\") {
                        if (xContent > yContent) {
                            shouldSwitch = true;
                            break;
                        }
                    } else if (dir == \"desc\") {
                        if (xContent < yContent) {
                            shouldSwitch = true;
                            break;
                        }
                    }
                }
                
                if (shouldSwitch) {
                    rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
                    switching = true;
                    switchcount++;
                } else {
                    if (switchcount == 0 && dir == \"asc\") {
                        dir = \"desc\";
                        switching = true;
                    }
                }
            }
        }

        function sortTable2(n) {
            var table = document.getElementById(\"betters\");
            var rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
            switching = true;
            dir = \"asc\";
            
            while (switching) {
                switching = false;
                rows = table.rows;
                
                for (i = 1; i < (rows.length - 1); i++) {
                    shouldSwitch = false;
                    x = rows[i].getElementsByTagName(\"TD\")[n];
                    y = rows[i + 1].getElementsByTagName(\"TD\")[n];
                    
                    var xContent = n > 0 ? parseFloat(x.innerHTML) : x.innerHTML.toLowerCase();
                    var yContent = n > 0 ? parseFloat(y.innerHTML) : y.innerHTML.toLowerCase();
                    
                    if (dir == \"asc\") {
                        if (xContent > yContent) {
                            shouldSwitch = true;
                            break;
                        }
                    } else if (dir == \"desc\") {
                        if (xContent < yContent) {
                            shouldSwitch = true;
                            break;
                        }
                    }
                }
                
                if (shouldSwitch) {
                    rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
                    switching = true;
                    switchcount++;
                } else {
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
            
            for (var i = 1; i < rows.length; i++) {
                var bettersColumn = rows[i].getElementsByTagName(\"TD\")[5];
                var bettersText = bettersColumn.innerHTML.toLowerCase();
                var searchName = betterName.toLowerCase().trim();
                
                if (bettersText.includes(searchName)) {
                    rows[i].style.display = \"\";
                    visibleCount++;
                } else {
                    rows[i].style.display = \"none\";
                }
            }
            
            var filterStatus = document.getElementById(\"filterStatus\");
            filterStatus.innerHTML = \"Showing \" + visibleCount + \" teams for <strong>\" + betterName + \"</strong>\";
            filterStatus.style.display = \"block\";
            
            var clickableNames = document.getElementsByClassName(\"clickable-name\");
            for (var i = 0; i < clickableNames.length; i++) {
                clickableNames[i].classList.remove(\"active-filter\");
            }
            
            var clickedElements = document.querySelectorAll('[onclick=\"filterByBetter(\\'' + betterName + '\\')\"]');
            if (clickedElements.length > 0) {
                clickedElements[0].classList.add(\"active-filter\");
            }
        }
        
        function showAllTeams() {
            var table = document.getElementById(\"teams\");
            var rows = table.rows;
            
            for (var i = 1; i < rows.length; i++) {
                rows[i].style.display = \"\";
            }
            
            document.getElementById(\"filterStatus\").style.display = \"none\";
            
            var clickableNames = document.getElementsByClassName(\"clickable-name\");
            for (var i = 0; i < clickableNames.length; i++) {
                clickableNames[i].classList.remove(\"active-filter\");
            }
        }

        document.getElementById(\"searchTeams\").addEventListener(\"input\", function(e) {
            var filter = e.target.value.toLowerCase();
            var table = document.getElementById(\"teams\");
            var rows = table.getElementsByTagName(\"tr\");
            
            for (var i = 1; i < rows.length; i++) {
                var teamName = rows[i].getElementsByTagName(\"TD\")[1];
                if (teamName) {
                    var txtValue = teamName.textContent || teamName.innerText;
                    if (txtValue.toLowerCase().indexOf(filter) > -1) {
                        rows[i].style.display = \"\";
                    } else {
                        rows[i].style.display = \"none\";
                    }
                }
            }
        });

        document.getElementById(\"searchBetters\").addEventListener(\"input\", function(e) {
            var filter = e.target.value.toLowerCase();
            var table = document.getElementById(\"betters\");
            var rows = table.getElementsByTagName(\"tr\");
            
            for (var i = 1; i < rows.length; i++) {
                var betterName = rows[i].getElementsByTagName(\"TD\")[0];
                if (betterName) {
                    var txtValue = betterName.textContent || betterName.innerText;
                    if (txtValue.toLowerCase().indexOf(filter) > -1) {
                        rows[i].style.display = \"\";
                    } else {
                        rows[i].style.display = \"none\";
                    }
                }
            }
        });
    </script>
</body>
</html>
";
