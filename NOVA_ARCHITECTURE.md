# NOVA — Architecture & Design Specification
### The AI-Native Office Suite for Everyone
**Version 1.0 — Written for AI Agent Implementation**

---

## What Is Nova

Nova is a fork of LibreOffice with a deeply integrated AI layer — not bolted on as an extension, but woven into the core experience the same way Cursor rewired VSCode. The analogy is exact: Cursor did not add an extension to VSCode. They rebuilt the product philosophy from the ground up, kept the powerful foundation, and made AI the primary way people interact with their tools.

Nova does the same for office software.

The target user is not a power user who knows Excel formulas. The target user is anyone who has something to say, something to calculate, something to present — but was always intimidated by office software. A student in Malawi who needs to write a CV. A small business owner who needs to track sales. A teacher who needs to make a class schedule. They know what they want. They have never known how to ask a computer for it.

Nova lets them ask in plain language and gets it done.

---

## What We Learned From Research

### Why Cursor Is Worth $29 Billion

Cursor did three things that VSCode extensions could never do:

1. **Codebase-level context** — it indexes your entire project and reasons across all files simultaneously, not just the one you have open
2. **Plan-then-act architecture** — it does not just suggest, it plans what it will do, shows you the plan, then executes across multiple files
3. **Shadow workspace** — it tests changes in the background before showing them to you, so suggestions are already validated

The lesson: deep integration beats bolted-on. Cursor's AI knows *where you are* in your project, *what you are trying to do*, and *what the consequences of an action are* before it acts.

### Why Microsoft Copilot Fails Users

Copilot has a 1.7/5 rating on Trustpilot. Microsoft CEO Satya Nadella himself described some integrations as "almost unusable." The reasons are consistent across complaints:

- **Feels tacked on** — the UI is a sidebar that does not connect to what you are doing on screen
- **Requires prompt skill** — users who do not know how to write good prompts get bad results, and Microsoft's response was to hire a company to teach users how to prompt
- **Context blindness** — Copilot often does not know what document you have open, what you have selected, or what you were just doing
- **Locked behind expensive subscriptions** — $21/user/month makes it inaccessible for most of the world

The lesson: AI that requires users to learn a new skill to use it has failed. The AI should adapt to the user, not the other way around.

### The Gap Nova Fills

Nobody has taken LibreOffice — a complete, production-grade, free office suite — and rebuilt it as an AI-first product. Microsoft Copilot requires Office 365. Google Workspace AI requires Google accounts and internet. Both require monthly subscriptions. Both are designed for knowledge workers in wealthy countries.

Nova is free. Nova works offline for document operations. Nova speaks plain language. Nova is for everyone.

---

## The Name

**Nova** — a star that suddenly increases dramatically in brightness. Something that was always there, now impossible to ignore.

It reflects the product philosophy: office software has always existed, but it was always for people who already knew how to use it. Nova makes it brilliant and accessible to everyone.

Do not name the AI after yourself. Here is why:

Naming the AI "Redson" ties the product's identity to you personally, which creates problems when you want to sell, partner, or step back. It also sounds like a personal assistant rather than a product feature. Instead, name the AI **Lumen** — light, clarity, illumination. It fits the Nova metaphor and is globally neutral.

When users talk to the AI they say *"Ask Lumen"* or *"Lumen, create a budget table."* This is memorable, brandable, and does not tie the product to any one person.

---

## Software Name & Branding

```
Product: Nova
AI Name: Lumen
Tagline: "Tell it what you need."
Secondary: "Office software, finally for everyone."
```

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                        NOVA                             │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │              LibreOffice Core (C++)              │   │
│  │   Writer │ Calc │ Impress │ Draw │ Base          │   │
│  │   (All document operations, rendering, formats)  │   │
│  └────────────────────┬────────────────────────────┘   │
│                       │ UNO API Bridge                  │
│  ┌────────────────────▼────────────────────────────┐   │
│  │              Lumen Agent Layer (Rust)            │   │
│  │                                                  │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │   │
│  │  │ Context  │  │ Planner  │  │  Executor    │  │   │
│  │  │ Engine   │  │          │  │              │  │   │
│  │  └──────────┘  └──────────┘  └──────────────┘  │   │
│  │                                                  │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │   │
│  │  │ Memory   │  │ Tool     │  │  Verifier    │  │   │
│  │  │ Store    │  │ Registry │  │              │  │   │
│  │  └──────────┘  └──────────┘  └──────────────┘  │   │
│  └────────────────────┬────────────────────────────┘   │
│                       │                                 │
│  ┌────────────────────▼────────────────────────────┐   │
│  │           Lumen UI Panel (C++ / VCL)             │   │
│  │   Chat input │ Action preview │ History          │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## Fork Strategy — How To Fork Without Destroying The Foundation

### What Cursor Taught Us About Forking

Cursor did not rewrite VSCode's rendering engine. They did not touch the text editing core. They added their AI layer on top and changed only what needed to change for the user experience. This is why their fork stays maintainable — they can pull upstream VSCode updates without massive conflicts.

Nova must follow the same discipline.

### The Rules of a Good Fork

**Rule 1: Minimum touch on core modules**
Never modify `sc/` (Calc core), `sw/` (Writer core), or `sd/` (Impress core) unless absolutely necessary. These contain decades of complex C++ that handles file format compatibility, formula evaluation, and rendering. Touch them as little as possible.

**Rule 2: All AI work lives in new modules**
Create a new top-level module `lumen/` that contains all AI code. This module depends on LibreOffice, not the other way around. This means upstream updates can be pulled with minimal conflicts.

**Rule 3: Connect through UNO, not through hacks**
LibreOffice's UNO API is the official programmatic interface for controlling Writer, Calc, and Impress from code. Use it. Do not hook into internal C++ APIs that are not part of UNO — those change between versions and will break your fork.

**Rule 4: UI changes through sfx2 and VCL only**
The `sfx2` module controls sidebars, toolbars, and the shell around documents. The `vcl` module is the widget toolkit. Make UI changes here. Do not scatter UI modifications across every module.

**Rule 5: Track upstream**
Set up a GitHub Action that runs weekly and checks if LibreOffice upstream has commits that conflict with your changes. Deal with conflicts early, not months later.

**Rule 6: Brand separately, not deeply**
Change the name, colors, splash screen, and about dialog. Do not rename internal C++ namespaces or classes — that creates massive diff noise that makes upstream merges painful.

### What To Change

```
CHANGE:
- Application name: LibreOffice → Nova
- Splash screen
- About dialog
- Default color theme (modernize the UI)
- Sidebar: add Lumen panel to sfx2
- Toolbar: add Lumen quick-action button
- Menu: add Lumen menu items
- Default font and spacing (modernize defaults)

DO NOT CHANGE:
- Core document engines (sc, sw, sd internals)
- File format filters
- UNO API implementation
- Formula engine
- Rendering pipeline
```

---

## The Lumen Agent Layer — Full Architecture

### The Core Loop (Perceive → Plan → Act → Verify → Show)

This is the architecture that separates Nova from Copilot. Copilot responds. Lumen acts. The difference is a loop.

```
User says: "make a budget table for my salary and expenses"

1. PERCEIVE
   - What document is open? (Calc spreadsheet, empty)
   - What is selected? (Cell A1)
   - What has the user done recently? (Just opened the file)
   - What is the user's history with this type of task? (First time)

2. PLAN
   - Lumen creates a structured plan before touching anything:
     Plan: {
       steps: [
         "Create header row: Month, Income, Rent, Food, Transport, Other, Total",
         "Add 12 rows for January through December",
         "Add SUM formula in Total column for each row",
         "Add a summary row at the bottom",
         "Apply basic formatting: bold headers, currency format"
       ],
       estimated_cells_affected: 91,
       reversible: true
     }
   - Show plan to user BEFORE executing

3. USER CONFIRMS (or edits the plan)
   - "Yes, do it" → proceed
   - "Also add a savings row" → update plan, re-show
   - "Cancel" → nothing touched

4. ACT
   - Execute each step via UNO API
   - Each step is atomic and logged
   - If a step fails, roll back to last checkpoint

5. VERIFY
   - Check that the result matches the plan
   - Run formula validation on any cells with formulas
   - Flag anything that looks wrong

6. SHOW
   - Highlight what changed (like Cursor's diff view)
   - Show summary: "Created budget table with 12 months and expense tracking"
   - Offer follow-up actions: "Want me to make a chart of this data?"
```

### The Five Components of Lumen

#### 1. Context Engine

The Context Engine is what makes Lumen feel like it understands you, not just your words.

It maintains:

```rust
struct DocumentContext {
    // What is open right now
    active_document: DocumentType,  // Writer, Calc, Impress
    active_sheet: Option<String>,   // For Calc: which sheet
    selection: Selection,           // What the user has selected
    cursor_position: Position,      // Where the cursor is
    
    // What the document contains
    document_summary: String,       // AI-generated summary of document content
    named_ranges: Vec<String>,      // For Calc: named ranges
    styles_in_use: Vec<String>,     // For Writer: paragraph styles used
    
    // What the user has been doing
    recent_actions: VecDeque<Action>,  // Last 20 actions
    session_goal: Option<String>,       // Inferred from conversation
    
    // Who the user is
    user_profile: UserProfile,
}

struct UserProfile {
    skill_level: SkillLevel,    // Inferred: Beginner, Intermediate, Expert
    preferred_language: String,
    common_tasks: Vec<String>,  // Tasks they do often
    formula_knowledge: bool,    // Do they know formulas?
}
```

The Context Engine feeds all of this into every prompt sent to the AI. Lumen always knows what you are looking at, not just what you said.

#### 2. Planner

The Planner is the brain. It takes the user's intent and produces a structured, reversible action plan before touching anything.

Key principle: **Never act without a plan. Always show the plan before acting.**

This is what Cursor's Composer Mode does. This is what Microsoft Copilot does not do. Copilot just acts. Users hate not knowing what is about to happen to their document.

```rust
struct ActionPlan {
    id: Uuid,
    user_intent: String,           // What the user said
    interpreted_goal: String,      // What Lumen thinks they mean
    steps: Vec<PlanStep>,
    estimated_impact: ImpactLevel, // Minor / Moderate / Major
    reversible: bool,
    checkpoint_before: DocumentSnapshot,
}

struct PlanStep {
    description: String,           // Human readable: "Create header row"
    uno_operations: Vec<UnoOp>,    // Actual operations to execute
    can_fail_safely: bool,         // If this fails, can we continue?
}
```

#### 3. Tool Registry

Lumen has a set of tools it can call, each mapped to UNO API operations. The AI generates a tool call, the Tool Registry validates it, and the Executor runs it.

```rust
// Writer tools
tool: write_text(position, text, style)
tool: format_selection(bold, italic, font_size, color)
tool: insert_table(rows, cols, headers)
tool: apply_style(style_name, range)
tool: create_toc()
tool: find_and_replace(find, replace)
tool: insert_image(path, width, height)

// Calc tools
tool: write_cell(row, col, value)
tool: write_formula(row, col, formula)
tool: format_range(row_start, col_start, row_end, col_end, format)
tool: create_chart(data_range, chart_type, title)
tool: create_named_range(name, range)
tool: sort_range(range, column, ascending)
tool: filter_range(range, column, condition)
tool: insert_pivot_table(data_range, rows, cols, values)

// Impress tools
tool: add_slide(layout, position)
tool: write_slide_title(slide_index, title)
tool: write_slide_content(slide_index, content)
tool: apply_theme(theme_name)
tool: add_image_to_slide(slide_index, image_path)
tool: set_transition(slide_index, transition_type)

// Cross-app tools
tool: export_as(format)   // pdf, docx, xlsx, pptx
tool: read_document()     // returns document content for AI reasoning
tool: take_snapshot()     // checkpoint for rollback
tool: rollback(checkpoint_id)
```

#### 4. Memory Store

Lumen remembers things about you across sessions. This is what Microsoft's "Work IQ" tries to do — and what they charge for. Nova includes it by default.

```rust
struct MemoryStore {
    // Persisted to local SQLite
    user_preferences: HashMap<String, Value>,
    common_phrases: Vec<String>,      // Things the user says often
    document_history: Vec<DocumentSummary>,
    correction_log: Vec<Correction>,  // When user corrected Lumen
    template_library: Vec<Template>,  // Documents the user uses as starting points
}
```

When a user corrects Lumen — "No, I wanted it in the first column, not the second" — Lumen logs this correction and uses it to inform future actions. It gets better the more you use it.

#### 5. Verifier

Before showing results, the Verifier checks:

- For Calc: are all formulas valid? Do they reference existing cells? Do they produce expected types?
- For Writer: are styles valid? Are references intact?
- For Impress: are slide layouts consistent?

If verification fails, Lumen rolls back and tells the user what went wrong in plain language — not an error code.

---

## The Lumen UI Panel

The UI lives in a sidebar panel in `sfx2`. It looks and feels different from the rest of LibreOffice — modern, clean, warm.

### Layout

```
┌─────────────────────────────┐
│  ✦ Lumen                    │
├─────────────────────────────┤
│                             │
│  [Document summary here]    │
│  "Empty spreadsheet"        │
│                             │
├─────────────────────────────┤
│  Recent:                    │
│  • Made budget table        │
│  • Added SUM formulas       │
│  • Formatted headers        │
├─────────────────────────────┤
│                             │
│  What do you need?          │
│                             │
│  ┌─────────────────────┐   │
│  │                     │   │
│  └─────────────────────┘   │
│              [Send]         │
│                             │
│  Quick actions:             │
│  [Format this] [Summarize]  │
│  [Make a chart] [Fix this]  │
│                             │
└─────────────────────────────┘
```

### Interaction Modes

**Mode 1: Chat** — user types naturally, Lumen responds and acts

**Mode 2: Selection-aware** — user selects cells or text, right-clicks, and sees Lumen options: "Explain this formula", "Format this selection", "Summarize this text"

**Mode 3: Inline suggestions** — while typing in a cell or document, Lumen notices patterns and offers to complete them. User presses Tab to accept, Escape to dismiss. Exactly like Cursor's tab completion.

**Mode 4: Document Q&A** — user asks questions about their own document. "What is the total in column B?" or "How many times does 'overdue' appear?" Lumen reads the document and answers.

---

## AI Model Strategy

### The Multi-Model Approach

Do not lock Nova to one AI provider. Cursor's biggest advantage over GitHub Copilot is model flexibility. Do the same.

```rust
enum ModelProvider {
    Cerebras,    // Free tier: 1M tokens/day, very fast — default for most operations
    Groq,        // Fast inference, free tier — fallback
    Gemini,      // Google's model, generous free tier — for complex reasoning
    OpenRouter,  // Access to many models — for users who want choice
    Local,       // Ollama running locally — for offline/private mode
}

struct ModelRouter {
    // Route based on task complexity and user preference
    fn route(task: &Task, user: &UserProfile) -> ModelProvider {
        match task.complexity {
            Complexity::Simple => ModelProvider::Cerebras,   // Fast, free
            Complexity::Medium => ModelProvider::Groq,
            Complexity::Complex => ModelProvider::Gemini,
            Complexity::Sensitive => ModelProvider::Local,   // Private docs
        }
    }
}
```

### Free Tier Strategy

Nova is free. The AI operations behind it use free tiers. This is your competitive moat against Microsoft Copilot at $21/month.

- Cerebras: 1M tokens/day, no credit card, unlimited requests per minute
- Groq: generous free tier, very fast
- Gemini: 1M context window, generous free tier
- Ollama: fully local, no API calls at all for users who want privacy

For heavy users, offer Nova Pro at $5/month — much cheaper than Copilot — which gives priority routing and higher limits.

---

## Build System & CI/CD

### Why GitHub Actions Solves The Build Problem

LibreOffice takes hours to compile. On a 4GB RAM machine this is impossible. GitHub Actions gives you free CI/CD with powerful runners that can build LibreOffice in the cloud.

### GitHub Actions Workflow

```yaml
# .github/workflows/build.yml
name: Nova Build

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: '0 2 * * 0'  # Weekly upstream sync check

jobs:
  build-linux:
    runs-on: ubuntu-22.04
    
    steps:
      - name: Checkout Nova
        uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Cache build artifacts
        uses: actions/cache@v4
        with:
          path: |
            ~/.ccache
            instdir/
          key: nova-build-${{ runner.os }}-${{ hashFiles('**/configure.ac') }}
          restore-keys: |
            nova-build-${{ runner.os }}-
      
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            build-essential git autoconf automake \
            libtool pkg-config nasm \
            libx11-dev libxt-dev \
            openjdk-17-jdk \
            python3-dev \
            ccache
      
      - name: Configure build
        run: |
          ./autogen.sh \
            --with-distro=LibreOfficeFull \
            --enable-ccache \
            --without-java \
            --disable-postgresql-sdbc \
            --disable-firebird-sdbc
      
      - name: Build Nova
        run: |
          make -j$(nproc)
      
      - name: Build Lumen module
        run: |
          cd lumen/
          cargo build --release
      
      - name: Run tests
        run: |
          make check
      
      - name: Package
        run: |
          make distro-pack-install
      
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: nova-linux-${{ github.sha }}
          path: instdir/
          retention-days: 7

  check-upstream:
    runs-on: ubuntu-22.04
    if: github.event_name == 'schedule'
    steps:
      - name: Check for upstream conflicts
        run: |
          git remote add upstream https://github.com/LibreOffice/core.git
          git fetch upstream
          git log HEAD..upstream/master --oneline | head -20
```

### Build Optimization

Use `ccache` — it caches compilation results so subsequent builds are dramatically faster. First build: ~3-4 hours on GitHub Actions. Subsequent builds with cache: 15-30 minutes.

---

## Module Structure

```
nova/                          ← Root (forked from LibreOffice/core)
├── lumen/                     ← NEW: All AI code lives here
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── agent/
│   │   │   ├── context.rs     ← Context Engine
│   │   │   ├── planner.rs     ← Planner
│   │   │   ├── executor.rs    ← Executor
│   │   │   ├── verifier.rs    ← Verifier
│   │   │   └── memory.rs      ← Memory Store
│   │   ├── tools/
│   │   │   ├── writer.rs      ← Writer UNO tools
│   │   │   ├── calc.rs        ← Calc UNO tools
│   │   │   ├── impress.rs     ← Impress UNO tools
│   │   │   └── registry.rs    ← Tool Registry
│   │   ├── models/
│   │   │   ├── router.rs      ← Model Router
│   │   │   ├── cerebras.rs
│   │   │   ├── groq.rs
│   │   │   ├── gemini.rs
│   │   │   └── local.rs       ← Ollama
│   │   └── bridge/
│   │       └── uno.rs         ← UNO API bridge from Rust to C++
│   └── tests/
│
├── sfx2/                      ← MODIFIED: Add Lumen sidebar panel
│   └── source/
│       └── sidebar/
│           └── LumenPanel.*   ← New sidebar panel
│
├── vcl/                       ← MODIFIED: Nova visual theme
│   └── ...
│
├── shell/                     ← MODIFIED: Nova branding
│   └── ...
│
└── [all other LibreOffice modules — untouched]
```

---

## The User Experience — What This Feels Like

### For the student writing their first CV

They open Nova. They see a blank Writer document. In the sidebar, Lumen says: *"What are you working on today?"*

They type: *"I need to write a CV to apply for a job at a bank."*

Lumen says: *"I'll create a professional CV for you. I'll need a few things — your name, what job you're applying for, your education, and any work experience. Tell me and I'll format everything properly."*

The student types naturally. Lumen builds the CV in real time. The student has never heard of paragraph styles or margins. They do not need to.

### For the shopkeeper tracking daily sales

They open Nova with a blank Calc spreadsheet. They type: *"I want to track my sales every day, I sell groceries."*

Lumen says: *"Here's a plan: I'll create columns for Date, Item, Quantity, Unit Price, and Total, with automatic calculations. I'll also add a weekly summary at the bottom. Should I do this?"*

The shopkeeper says yes. Lumen builds it. The shopkeeper has never heard of SUM formulas. They do not need to.

### For the teacher making a class schedule

They open Nova with a blank Impress presentation. They type: *"Make me 5 slides for a lesson about the water cycle."*

Lumen plans: 5 slides with titles, bullet points, and a consistent design. Shows the plan. User confirms. Lumen builds it.

---

## What Makes Nova Different From Copilot

| Feature | Microsoft Copilot | Nova (Lumen) |
|---|---|---|
| Price | $21/user/month | Free |
| Plan before acting | No — just acts | Yes — always shows plan |
| Context awareness | Weak | Deep — knows document, selection, history |
| Works offline | No | Yes (Ollama mode) |
| Model choice | Locked to Microsoft | Cerebras, Groq, Gemini, Local |
| Designed for beginners | No | Yes — primary design goal |
| Available globally | Restricted by subscription | Free for everyone |
| Learns from corrections | Memory feature costs extra | Built in by default |

---

## Phases of Development

### Phase 1 — Foundation (Weeks 1-4)
- Fork LibreOffice on GitHub
- Set up GitHub Actions build pipeline
- Confirm build succeeds in CI
- Apply Nova branding (name, colors, splash screen)
- Add empty Lumen sidebar panel in sfx2 (no AI yet, just the UI shell)
- **Goal: Nova builds, runs, shows Lumen panel**

### Phase 2 — Lumen Core (Weeks 5-10)
- Build the Rust `lumen/` module
- Implement UNO bridge from Rust to LibreOffice
- Implement Tool Registry with basic tools (write cell, write text, format)
- Implement basic chat loop: user types → AI responds → Lumen executes
- Integrate Cerebras API as default model
- **Goal: User can type "add a title" and Lumen does it**

### Phase 3 — Context & Planning (Weeks 11-16)
- Implement Context Engine (document awareness)
- Implement Planner (plan before act)
- Implement Verifier (check before show)
- Implement Memory Store (SQLite, persists across sessions)
- Add all tool categories (Writer, Calc, Impress)
- **Goal: Full plan-then-act loop working across all three apps**

### Phase 4 — Polish & Ship (Weeks 17-20)
- Inline suggestion mode (tab to accept)
- Selection-aware right-click menu
- Document Q&A mode
- Multi-model routing
- Local Ollama support
- Packaging: .deb, .rpm, Windows installer, macOS .dmg
- **Goal: Shippable product**

---

## How To Give This To An AI Agent

When giving this document to a coding agent, give it one phase at a time. Never give Phase 3 work before Phase 1 is confirmed working.

For each phase, give the agent:
1. This architecture document
2. The specific phase goal
3. The LibreOffice module it needs to modify
4. The expected output to test against

Start with: *"Using the Nova Architecture document, implement Phase 1. Fork LibreOffice core, set up the GitHub Actions workflow, and add an empty Lumen sidebar panel in sfx2. The panel should show the text 'Lumen is ready' and nothing else. The build must succeed in CI."*

Do not ask the agent to implement everything at once. Cursor was not built in a day. Build the foundation first.

---

## Final Note

The most important architectural decision in this document is not technical. It is the one stated at the beginning: Nova is for people who were always intimidated by office software.

Every technical decision flows from that. The plan-before-act loop exists because users need to trust what the AI is doing. The free model strategy exists because the target user cannot afford $21/month. The beginner-first UX exists because the user does not know what a formula is.

When a technical decision comes up that is not covered in this document, return to that principle and ask: *"Does this make Nova easier and more trustworthy for someone who has never used office software before?"*

If yes, do it. If no, find another way.

---

*Architecture by Redson Ngwira, Lilongwe, Malawi, 2026.*
*Built to prove that world-class technology can come from anywhere.*
