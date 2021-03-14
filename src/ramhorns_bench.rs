use criterion;
use liquid::{model::Value, Object, ParserBuilder};
use ramhorns::{Content, Template};

#[derive(Content)]
struct BigTable {
    table: Vec<Vec<usize>>,
}

#[derive(Content)]
struct Teams {
    year: u16,
    teams: Vec<Team>,
}

#[derive(Content)]
struct TeamsTemplateData {
    year: u16,
    teams: Vec<Team>,
    champion: Team
}

#[derive(Content)]
struct Team {
    name: String,
    score: u8,
}

static BIG_TABLE_TEMPLATE: &'static str = "<table>\
{{#table}}\
<tr>{{#.}}<td>{{.}}</td>{{/.}}</tr>\
{{/table}}\
</table>";

static TEAMS_TEMPLATE: &'static str = "<html>\
<head>\
  <title>{{ year }}</title>\
</head>\
<body>\
  <h1>CSL {{ year }}</h1>\
  <ul>\
  <li class=\"champion\">\
    <b>{{ champion.name }}</b>: {{ champion.score }}\
    </li>\
  {{#teams}}\
    <li>\
    <b>{{ name }}</b>: {{ score }}\
    </li>\
  {{/teams}}\
  </ul>\
</body>\
</html>";

/*
{
    "table": [
	[1, 2, 3],
	[3, 4, 5],
	[6, 7, 8],
	[9, 10, 11, 12, 14]
    ]
}

<table>
{{#table}}
<tr>{{#.}}<td>{{.}}</td>{{/.}}</tr>
{{/table}}
</table>
*/

pub fn big_table(b: &mut criterion::Bencher<'_>, size: &usize) {
    let mut table = Vec::with_capacity(*size);
    for _ in 0..*size {
        let mut inner = Vec::with_capacity(*size);
        for i in 0..*size {
            inner.push(Value::Scalar((i as i32).into()));
        }
        table.push(Value::Array(inner));
    }

    let data = BigTable {
        table
    };

    let template = Template::new(BIG_TABLE_TEMPLATE).unwrap();

    b.iter(|| template.render(&data {

    }));
}

/*
{
  "year": 2015,
  "teams": [
    {
      "name": "Jiangsu",
      "score": 43
    },
    {
      "name": "Beijing",
      "score": 27
    },
    {
      "name": "Guangzhou",
      "score": 22
    },
    {
      "name": "Shandong",
      "score": 12
    }
  ]
}

<html>
  <head>
    <title>{{ year }}</title>
  </head>
  <body>
    <h1>CSL {{ year }}</h1>
    <ul>
  <li class="champion">
    <b>{{ champion.name }}</b>: {{ champion.score }}
    </li>
    {{#teams}}
      <li>
      <b>{{ name }}</b>: {{ score }}
      </li>
    {{/teams}}
    </ul>
  </body>
</html>
*/

pub fn teams(b: &mut criterion::Bencher<'_>, _: &usize) {
    let mut teams = Teams {
      year: 2015,
      teams: vec![
          Team {
              name: "Jiangsu".into(),
              score: 43,
          },
          Team {
              name: "Beijing".into(),
              score: 27,
          },
          Team {
              name: "Guangzhou".into(),
              score: 22,
          },
          Team {
              name: "Shandong".into(),
              score: 12,
          },
      ],
  };
    let champion: Team = teams.teams.split_off(0);
    let data: TeamsTemplateData = TeamsTemplateData {
        year: teams_data.year,
        teams: teams_data.teams,
        champion
    };
    let template = Template::new(data).unwrap();

    b.iter(|| template.render(&TEAMS_DATA {

    }));
}

