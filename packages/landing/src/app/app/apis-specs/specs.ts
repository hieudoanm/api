// Must match basePath in next.config.ts (GitHub Pages project site).
// Raw hrefs don't get basePath applied automatically.
const BASE_PATH = process.env.NODE_ENV === 'production' ? '/api' : '';

export interface ApiSpec {
  file: string;
  name: string;
  tagline: string;
  category: string;
  endpoints: number;
}

export const SPECS: ApiSpec[] = [
  {
    file: 'api.chess.com.yaml',
    name: 'Chess.com',
    tagline:
      'Published Data API — players, games, clubs, puzzles and leaderboards.',
    category: 'Gaming',
    endpoints: 30,
  },
  {
    file: 'lichess.org.api.yaml',
    name: 'Lichess',
    tagline:
      'Open-source chess platform — games, puzzles, tournaments, TV and more.',
    category: 'Gaming',
    endpoints: 190,
  },
  {
    file: 'api.openai.com.yaml',
    name: 'OpenAI',
    tagline:
      'Official spec — chat, responses, assistants, audio, realtime and more.',
    category: 'AI',
    endpoints: 388,
  },
  {
    file: 'api.generativelanguage.googleapis.com.yaml',
    name: 'Gemini',
    tagline:
      'Google Gemini API — generateContent, streaming, batch and tuned models.',
    category: 'AI',
    endpoints: 50,
  },
  {
    file: 'api.anthropic.com.yaml',
    name: 'Claude',
    tagline:
      'Anthropic Messages API — messages, batches, files and organizations.',
    category: 'AI',
    endpoints: 14,
  },
  {
    file: 'api.deepseek.com.yaml',
    name: 'DeepSeek',
    tagline: 'OpenAI-compatible chat and legacy completions API.',
    category: 'AI',
    endpoints: 4,
  },
  {
    file: 'api.telegram.org.yaml',
    name: 'Telegram Bot',
    tagline:
      'Telegram Bot API — 135 methods for messages, media, payments and more.',
    category: 'Messaging',
    endpoints: 135,
  },
  {
    file: 'newsapi.org.yaml',
    name: 'NewsAPI',
    tagline:
      'Live news articles — search everything, top headlines and sources.',
    category: 'News',
    endpoints: 3,
  },
  {
    file: 'api.openweathermap.org.yaml',
    name: 'OpenWeatherMap',
    tagline: 'Current weather, forecasts, air pollution and weather stations.',
    category: 'Weather',
    endpoints: 7,
  },
  {
    file: 'api.airvisual.com.v2.yaml',
    name: 'AirVisual',
    tagline: 'IQAir air quality data — cities, stations and rankings.',
    category: 'Weather',
    endpoints: 9,
  },
  {
    file: 'api.frankfurter.dev.v2.yaml',
    name: 'Frankfurter v2',
    tagline: 'Free exchange-rate API — current, historical and provider rates.',
    category: 'Finance',
    endpoints: 9,
  },
  {
    file: 'api.frankfurter.dev.v1.yaml',
    name: 'Frankfurter v1',
    tagline:
      'Legacy Frankfurter endpoints — latest, date ranges and currencies.',
    category: 'Finance',
    endpoints: 5,
  },
  {
    file: 'data.fixer.io.api.yaml',
    name: 'Fixer.io',
    tagline: 'Exchange rates — latest, historical, timeseries and conversion.',
    category: 'Finance',
    endpoints: 6,
  },
  {
    file: 'api.coinranking.com.v2.yaml',
    name: 'Coinranking',
    tagline: 'Crypto market data — coins, exchanges, markets and statistics.',
    category: 'Crypto',
    endpoints: 34,
  },
  {
    file: 'api.football.data.org.yaml',
    name: 'football-data.org',
    tagline:
      'Football/soccer data — competitions, matches, teams and standings.',
    category: 'Sports',
    endpoints: 15,
  },
  {
    file: 'api.crossref.org.yaml',
    name: 'Crossref',
    tagline:
      'Scholarly metadata — works, journals, funders, members and prefixes.',
    category: 'Research',
    endpoints: 18,
  },
  {
    file: 'api.github.com.json',
    name: 'GitHub',
    tagline:
      'Official REST API — repos, issues, pull requests, actions and more.',
    category: 'Developer',
    endpoints: 1281,
  },
  {
    file: 'api.stripe.com.json',
    name: 'Stripe',
    tagline:
      'Official spec — payments, subscriptions, invoices, checkout and more.',
    category: 'Payments',
    endpoints: 612,
  },
  {
    file: 'api.paypal.com.json',
    name: 'PayPal',
    tagline: 'Payments v2 — orders, captures, refunds and disputes.',
    category: 'Payments',
    endpoints: 8,
  },
  {
    file: 'api.twilio.com.json',
    name: 'Twilio',
    tagline: 'Communications APIs — messaging, voice, video, verify and more.',
    category: 'Communication',
    endpoints: 197,
  },
  {
    file: 'api.bitbucket.org.json',
    name: 'Bitbucket',
    tagline:
      'Official spec — repositories, pull requests, pipelines and workspaces.',
    category: 'Developer',
    endpoints: 300,
  },
  {
    file: 'api.atlassian.net.jira.json',
    name: 'Jira',
    tagline: 'Jira Cloud REST API — issues, boards, sprints, users and search.',
    category: 'Productivity',
    endpoints: 619,
  },
  {
    file: 'api.atlassian.net.confluence.json',
    name: 'Confluence',
    tagline: 'Confluence Cloud REST API — pages, spaces, comments and search.',
    category: 'Productivity',
    endpoints: 130,
  },
  {
    file: 'api.asana.com.yaml',
    name: 'Asana',
    tagline: 'Official spec — tasks, projects, portfolios, goals and webhooks.',
    category: 'Productivity',
    endpoints: 271,
  },
  {
    file: 'api.supabase.com.json',
    name: 'Supabase',
    tagline:
      'Backend platform — projects, auth, database, edge functions and storage.',
    category: 'Developer',
    endpoints: 116,
  },
  {
    file: 'api.meilisearch.com.yaml',
    name: 'Meilisearch',
    tagline:
      'Lightning-fast search engine — indexes, documents, keys and dumps.',
    category: 'Search',
    endpoints: 36,
  },
  {
    file: 'api.qdrant.io.json',
    name: 'Qdrant',
    tagline:
      'Vector database — collections, points, search, snapshots and aliases.',
    category: 'AI',
    endpoints: 53,
  },
  {
    file: 'api.replicate.com.json',
    name: 'Replicate',
    tagline:
      'Run machine learning models in the cloud — predictions, deployments and hardware.',
    category: 'AI',
    endpoints: 26,
  },
  {
    file: 'api.todoist.com.json',
    name: 'Todoist',
    tagline: 'Task manager — tasks, projects, sections, comments and activity.',
    category: 'Productivity',
    endpoints: 94,
  },
  {
    file: 'coda.io.json',
    name: 'Coda',
    tagline:
      'Docs and apps platform — docs, pages, tables, formulas and automations.',
    category: 'Productivity',
    endpoints: 90,
  },
  {
    file: 'api.weather.gov.json',
    name: 'National Weather Service',
    tagline:
      'U.S. weather.gov API — forecasts, alerts, observations and radar.',
    category: 'Weather',
    endpoints: 69,
  },
  {
    file: 'api.spotify.com.yaml',
    name: 'Spotify',
    tagline:
      'Spotify Web API — search, playback, playlists, library and recommendations.',
    category: 'Media',
    endpoints: 67,
  },
  {
    file: 'api.nasa.gov.yaml',
    name: 'NASA Open APIs',
    tagline:
      'Astronomy Picture of the Day, near-Earth asteroids, space weather and Earth imagery.',
    category: 'Science',
    endpoints: 11,
  },
  {
    file: 'dummyjson.com.json',
    name: 'DummyJSON',
    tagline:
      'Fake REST API with realistic data — products, carts, users, quotes and more.',
    category: 'Developer',
    endpoints: 68,
  },
  {
    file: 'reqres.in.json',
    name: 'ReqRes',
    tagline:
      'Hosted fake REST API for testing — users, posts, comments and delayed responses.',
    category: 'Developer',
    endpoints: 43,
  },
  {
    file: 'openlibrary.org.json',
    name: 'Open Library',
    tagline:
      'Internet Archive book catalog — search, works, authors, subjects and lists.',
    category: 'Books',
    endpoints: 50,
  },
  {
    file: 'restcountries.com.json',
    name: 'REST Countries',
    tagline:
      'Country data — capitals, currencies, languages, flags and regions.',
    category: 'Geography',
    endpoints: 13,
  },
  {
    file: 'trello.com.yaml',
    name: 'Trello',
    tagline:
      'Official spec — boards, lists, cards, checklists, labels and members.',
    category: 'Productivity',
    endpoints: 264,
  },
  {
    file: 'api.rawg.io.yaml',
    name: 'RAWG',
    tagline:
      'Video game database — games, genres, platforms, stores and developers.',
    category: 'Gaming',
    endpoints: 30,
  },
  {
    file: 'api.spoonacular.com.yaml',
    name: 'Spoonacular',
    tagline:
      'Food and recipe API — recipes, ingredients, meal plans and nutrition.',
    category: 'Food',
    endpoints: 97,
  },
  {
    file: 'soundcloud.com.yaml',
    name: 'SoundCloud',
    tagline:
      'Public API — tracks, playlists, users, likes, reposts and stream URLs.',
    category: 'Media',
    endpoints: 46,
  },
  {
    file: 'tvmaze.com.yaml',
    name: 'TVmaze',
    tagline: 'TV shows — search, episodes, cast, schedules and user follows.',
    category: 'Entertainment',
    endpoints: 25,
  },
  {
    file: 'worldtimeapi.org.yaml',
    name: 'WorldTimeAPI',
    tagline:
      'Time zones — current time, offsets and daylight-saving transitions.',
    category: 'Utilities',
    endpoints: 12,
  },
  {
    file: 'dnd5eapi.co.yaml',
    name: 'D&D 5e API',
    tagline:
      'Dungeons & Dragons — monsters, spells, classes, equipment and conditions.',
    category: 'Gaming',
    endpoints: 47,
  },
  {
    file: 'circleci.com.yaml',
    name: 'CircleCI',
    tagline: 'CI/CD REST API — projects, pipelines, workflows and artifacts.',
    category: 'Developer',
    endpoints: 17,
  },
  {
    file: 'api.digitalocean.com.yaml',
    name: 'DigitalOcean',
    tagline:
      'Cloud infrastructure — droplets, apps, databases, domains and billing.',
    category: 'Cloud',
    endpoints: 183,
  },
  {
    file: 'api.sendgrid.com.yaml',
    name: 'SendGrid',
    tagline:
      'Email API — send, templates, contacts, suppressions and email activity.',
    category: 'Communication',
    endpoints: 201,
  },
  {
    file: 'httpbin.org.json',
    name: 'HttpBin',
    tagline:
      'HTTP request testing service — headers, IPs, redirects, auth and more.',
    category: 'Developer',
    endpoints: 52,
  },
  {
    file: 'api.notion.com.yaml',
    name: 'Notion',
    tagline:
      'Workspace API — pages, databases, blocks, users, comments and search.',
    category: 'Productivity',
    endpoints: 8,
  },
  {
    file: 'docusign.net.yaml',
    name: 'DocuSign',
    tagline:
      'eSignature REST API — envelopes, templates, workspaces and connect.',
    category: 'Productivity',
    endpoints: 208,
  },
  {
    file: 'gitlab.com.json',
    name: 'GitLab',
    tagline:
      'DevOps platform — projects, merge requests, pipelines, issues and groups.',
    category: 'Developer',
    endpoints: 251,
  },
  {
    file: 'netlify.com.json',
    name: 'Netlify',
    tagline:
      'Web hosting — sites, deploys, forms, functions, DNS and split testing.',
    category: 'Cloud',
    endpoints: 75,
  },
  {
    file: 'api.box.com.yaml',
    name: 'Box',
    tagline:
      'Cloud content management — files, folders, collaborations and search.',
    category: 'Cloud',
    endpoints: 161,
  },
  {
    file: 'api.getpostman.com.yaml',
    name: 'Postman',
    tagline:
      'API platform — collections, environments, monitors, mocks and workspaces.',
    category: 'Developer',
    endpoints: 34,
  },
  {
    file: 'api.zoom.us.yaml',
    name: 'Zoom',
    tagline:
      'Video conferencing — meetings, webinars, users, recordings and chat.',
    category: 'Communication',
    endpoints: 265,
  },
  {
    file: 'squareup.com.yaml',
    name: 'Square',
    tagline:
      'Payments and commerce — payments, orders, customers, invoices and catalog.',
    category: 'Payments',
    endpoints: 159,
  },
  {
    file: 'mailchimp.com.yaml',
    name: 'Mailchimp',
    tagline:
      'Marketing platform — campaigns, audiences, automations and reports.',
    category: 'Marketing',
    endpoints: 166,
  },
  {
    file: 'api.polygon.io.json',
    name: 'Polygon.io',
    tagline:
      'Stock market data — aggregates, trades, quotes, tickers and reference.',
    category: 'Finance',
    endpoints: 10,
  },
  {
    file: 'api.trakt.tv.yaml',
    name: 'Trakt',
    tagline:
      'TV and movie tracking — shows, episodes, movies, users and lists.',
    category: 'Entertainment',
    endpoints: 170,
  },
  {
    file: 'api.giphy.com.yaml',
    name: 'GIPHY',
    tagline:
      'GIF search and trending — search, categories, stickers and random.',
    category: 'Media',
    endpoints: 10,
  },
  {
    file: 'api.tcgdex.net.yaml',
    name: 'TCGdex',
    tagline:
      'Trading card game database — cards, sets, series and illustrations.',
    category: 'Gaming',
    endpoints: 33,
  },
  {
    file: 'api.setlist.fm.json',
    name: 'setlist.fm',
    tagline:
      'Concert setlists — artists, events, venues, songs and tour history.',
    category: 'Music',
    endpoints: 15,
  },
  {
    file: 'api.weatherbit.io.json',
    name: 'Weatherbit',
    tagline:
      'Weather data — current, forecast, historical, air quality and severe alerts.',
    category: 'Weather',
    endpoints: 47,
  },
  {
    file: 'wikimedia.org.json',
    name: 'Wikimedia',
    tagline: 'Wikipedia and Wikimedia — page summaries, search and random.',
    category: 'Reference',
    endpoints: 35,
  },
];

// Specs are served from /apis/{json,yaml}/<file>.
export const specHref = (file: string): string =>
  `${BASE_PATH}/apis/${file.endsWith('.json') ? 'json' : 'yaml'}/${file}`;
