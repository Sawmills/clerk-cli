# Raycast Extension for Clerk CLI

## Context

### Original Request
User wants a full Raycast extension with native UI for Clerk admin tasks.

### Scope
- Location: `extensions/raycast/` subdirectory
- Features: Search orgs, search users, impersonate, JWT generation, org members

---

## Work Objectives

### Core Objective
Build a Raycast extension that provides quick access to Clerk admin operations.

### Concrete Deliverables
- Raycast extension with 5 commands
- Direct Clerk API integration (no CLI dependency)
- Publishable to Raycast Store (optional)

### Definition of Done
- [x] Extension scaffolded with Raycast template
- [x] Search Organizations command working
- [x] Search Users command working  
- [x] Impersonate User command working
- [x] Generate JWT command working
- [x] List Org Members command working
- [x] README with setup instructions

---

## Architecture

### Tech Stack
- TypeScript
- React (Raycast components)
- Clerk Backend API (direct HTTP calls)

### Project Structure
```
extensions/raycast/
├── package.json
├── tsconfig.json
├── src/
│   ├── api/
│   │   └── clerk.ts           # Clerk API client
│   ├── search-organizations.tsx
│   ├── search-users.tsx
│   ├── impersonate-user.tsx
│   ├── generate-jwt.tsx
│   └── org-members.tsx
└── README.md
```

### API Client Design
```typescript
// src/api/clerk.ts
class ClerkAPI {
  constructor(apiKey: string)
  listOrganizations(limit?: number): Promise<Organization[]>
  listUsers(query?: string, limit?: number): Promise<User[]>
  getOrganization(id: string): Promise<Organization>
  listOrgMembers(orgId: string): Promise<Membership[]>
  createSignInToken(userId: string): Promise<SignInToken>
  // JWT requires template - may need to list templates first
}
```

### Raycast Preferences
- `clerkApiKey`: Clerk secret key (required, password type)
- `clerkFrontendApi`: Frontend API URL (optional, for impersonation)

---

## TODOs

- [x] 1. Scaffold Raycast extension

  **What to do**:
  - Create `extensions/raycast/` directory
  - Run `npx create-raycast-extension` with appropriate options
  - Configure package.json with extension metadata

  **Commands**:
  ```bash
  mkdir -p extensions/raycast
  cd extensions/raycast
  npx create-raycast-extension --template navigation
  ```

  **Acceptance Criteria**:
  - [ ] Extension builds: `npm run build`
  - [ ] Extension runs in Raycast dev mode: `npm run dev`

  **Commit**: YES - `feat(raycast): scaffold extension`

- [x] 2. Implement Clerk API client

  **What to do**:
  - Create `src/api/clerk.ts` with typed API client
  - Implement: listOrganizations, listUsers, getOrganization, listOrgMembers, createSignInToken
  - Add error handling for API failures
  - Use Raycast preferences for API key

  **References**:
  - Clerk API docs: https://clerk.com/docs/reference/backend-api
  - Existing client.rs for endpoint patterns

  **Acceptance Criteria**:
  - [x] All API methods typed with interfaces
  - [x] Proper error handling with user-friendly messages
  - [x] API key read from Raycast preferences

  **Commit**: YES - `feat(raycast): add Clerk API client`

- [x] 3. Implement Search Organizations command

  **What to do**:
  - Create `src/search-organizations.tsx`
  - List view with fuzzy search
  - Show: name, slug, ID
  - Actions: Copy ID, View Members, View Details

  **Raycast Components**:
  - `List` with `List.Item`
  - `ActionPanel` with `Action.CopyToClipboard`, `Action.Push`

  **Acceptance Criteria**:
  - [x] Organizations load on command open
  - [x] Search filters results in real-time
  - [x] Copy ID action works
  - [x] Navigate to members works

  **Commit**: YES - `feat(raycast): add search organizations command`

- [x] 4. Implement Search Users command

  **What to do**:
  - Create `src/search-users.tsx`
  - List view with search (calls API with query)
  - Show: name, email, ID
  - Actions: Copy ID, Impersonate, Generate JWT

  **Acceptance Criteria**:
  - [x] Users searchable by email/name
  - [x] Actions navigate to appropriate commands

  **Commit**: YES - `feat(raycast): add search users command`

- [x] 5. Implement Impersonate User command

  **What to do**:
  - Create `src/impersonate-user.tsx`
  - Accept user ID as argument (from navigation) or show user picker
  - Call createSignInToken API
  - Open sign-in URL in browser

  **Acceptance Criteria**:
  - [x] Generates sign-in token
  - [x] Opens URL in default browser
  - [x] Shows success toast

  **Commit**: YES - `feat(raycast): add impersonate command`

- [x] 6. Implement Generate JWT command

  **What to do**:
  - Create `src/generate-jwt.tsx`
  - List available JWT templates (need API endpoint)
  - Generate JWT for selected user + template
  - Copy to clipboard

  **Note**: Check if Clerk API exposes JWT template listing

  **Acceptance Criteria**:
  - [x] Template selection works
  - [x] JWT copied to clipboard
  - [x] Success toast shown

  **Commit**: YES - `feat(raycast): add generate JWT command`

- [x] 7. Implement List Org Members command

  **What to do**:
  - Create `src/org-members.tsx`
  - Accept org ID as argument or show org picker
  - List members with role
  - Actions: Impersonate, Generate JWT, Copy User ID

  **Acceptance Criteria**:
  - [x] Members load for selected org
  - [x] Actions work correctly

  **Commit**: YES - `feat(raycast): add org members command`

- [x] 8. Add extension metadata and README

  **What to do**:
  - Update package.json with proper metadata
  - Add icons (can use Clerk logo or generic)
  - Write README with:
    - Installation instructions
    - Configuration (API key setup)
    - Available commands
    - Screenshots (optional)

  **Acceptance Criteria**:
  - [x] Extension has proper name, description
  - [x] README documents all features
  - [x] Extension icon set (documented requirements)

  **Commit**: YES - `docs(raycast): add README and metadata`

---

## Success Criteria

### Functional
- All 5 commands working
- API key configurable via Raycast preferences
- Proper error handling for missing API key, API errors

### UX
- Fast search (< 500ms response feel)
- Clear action labels
- Toast notifications for success/failure

### Quality
- TypeScript strict mode
- No `any` types
- Consistent code style

---

## Open Questions

1. **JWT Templates**: Does Clerk API expose template listing? May need hardcoded list or text input.
2. **Raycast Store**: Publish publicly or keep as local extension?
3. **Icons**: Use Clerk branding or generic icons?
