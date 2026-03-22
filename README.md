# NQ-MCP

Adding Natural Intelligence to AI conversations since 2026.

## Getting started

### Prerequisites

A git client, your operating system probably has git already or has a standard way to install it. If not then [this guide should help](https://git-scm.com/install/windows).

You will need an API key from OpenAQ. Navigate to [OpenAQ](https://openaq.org/) and create an account on their site. Your API Key will be on your [personal settings page](https://explore.openaq.org/account).

You must be able to run `rust` applications. Again your operating system probably has a standard way to get `rust` running but [this guide should be helpful](https://rust-lang.org/tools/install/).

NQ-MCP has been tested with Gemini CLI, follow these [instructions](https://geminicli.com/docs/get-started/installation/) to get Gemini CLI running for you.

### Steps

1. Configure Gemini CLI by adding the following configuration

```
    "mcpServers": {
	"localOpenAq": {
      "command": "cargo",
      "args": ["run"],
      "env": {
        "OPENAQ-API-KEY": "API_KEY_FROM_OPENAQ"
      },
      "cwd": "./",
      "timeout": 30000,
      "trust": true
	}
  }
```

to the `settings.json` file in the `.gemini` directory.

Note that if using an MCP Host other than Gemini CLI, the MCP server configuration should follow the docs of that tool.

2. Check out this repository from GitHub.

```bash
git clone https://github.com/matyjas/nq-mcp.git
```

3. Navigate to the nq-mcp folder that you just checked out.

```bash
cd nq-mcp
```

4. Start gemini

```bash
gemini
```

5. Start asking gemini about air quality

```bash
how is the air quality in Westminster, London right now?
```

Wait a bit and then

```bash
 The air quality in Westminster, London at the Marylebone Road station is as follows:
   - Nitrogen Dioxide (NO2): 20.7 µg/m³
   - PM2.5: 10.0 µg/m³
   - Ozone (O3): 70.45 µg/m³
   - PM10: 14.0 µg/m³
   - Sulfur Dioxide (SO2): 0.53 µg/m³
```
