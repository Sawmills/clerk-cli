# Learnings - Raycast Extension

## Task 2: Clerk API Client Implementation

### API Patterns Discovered

1. **Response Wrappers**:
   - Organizations endpoint returns: `{ data: Organization[] }`
   - Memberships endpoint returns: `{ data: Membership[] }`
   - Users endpoint returns: `User[]` directly (no wrapper)
   - Must handle both patterns in TypeScript client

2. **JWT Generation Flow**:
   - Requires two-step process:
     1. List user's active sessions: `GET /sessions?user_id={userId}&status=active`
     2. Create session token: `POST /sessions/{sessionId}/tokens/{templateName}`
   - Cannot directly create JWT without active session
   - Must handle "no active sessions" error case

3. **Error Handling Pattern**:
   - Clerk API returns: `{ errors: [{ message: string, code: string }] }`
   - Always use first error message for user-facing errors
   - Fallback to HTTP status text if JSON parsing fails

4. **TypeScript Conventions**:
   - Use `interface` for all data types (not `type`)
   - Use `class` for API client
   - Use `async/await` for all promises
   - Export helper function `getClerkClient()` for easy instantiation

5. **Raycast Preferences**:
   - Use `getPreferenceValues<Preferences>()` to read config
   - Preferences defined in package.json manifest
   - API key stored as `password` type (secure)
   - Frontend API optional for custom domains

### Code Organization

File structure:
```
src/api/clerk.ts
├── Preferences interface
├── Response type interfaces (User, Organization, etc.)
├── Error interfaces
├── Response wrapper interfaces
├── ClerkAPI class
│   ├── constructor(apiKey)
│   ├── private request<T>() - shared HTTP logic
│   ├── listOrganizations()
│   ├── listUsers()
│   ├── getOrganization()
│   ├── listOrgMembers()
│   ├── createSignInToken()
│   ├── listJwtTemplates()
│   └── createUserJwt() - complex two-step flow
├── getClerkClient() - helper factory
└── Display helpers (getUserDisplayName, etc.)
```

### TypeScript Best Practices Applied

1. **Strict typing**: No `any` types used
2. **Null safety**: All optional fields typed as `string | null`
3. **Generic request method**: Reusable `request<T>()` for all API calls
4. **Error propagation**: Throw typed errors, let caller handle
5. **Helper functions**: Extracted display logic for reuse

### Dependencies Used

- `@raycast/api`: For `getPreferenceValues()`
- Built-in `fetch()`: No external HTTP library needed
- TypeScript 5.2.2: Modern async/await support

### Verification Steps

1. ✅ TypeScript compilation: `npm run build` succeeds
2. ✅ LSP diagnostics: No errors or warnings
3. ✅ All 7 required methods implemented
4. ✅ All response types properly defined
5. ✅ Error handling matches Rust client behavior

## Task 3-8: Command Implementations

### React/TypeScript Compatibility Issue
- **Problem**: TypeScript 5.2 + @types/react 18.2.27 caused JSX component type errors
- **Solution**: Downgraded @types/react to 18.0.38
- **Root Cause**: Known incompatibility between React 18 types and TypeScript 5.2
- **Lesson**: Check @types/react version compatibility when using latest TypeScript

### Raycast Component Patterns
1. **List Component**: Primary UI for searchable lists
   - Use `throttle` prop for better performance
   - Use `List.EmptyView` for empty states
   - Use `accessories` for secondary info (IDs, roles, etc.)

2. **ActionPanel**: Container for actions
   - `Action.CopyToClipboard`: Copy text to clipboard
   - `Action.Push`: Navigate to another view
   - `Action.onAction`: Execute immediate action

3. **Navigation**: Use `Action.Push` with component instances
   ```typescript
   <Action.Push target={<Component prop={value} />} />
   ```

4. **Toast Notifications**: Three styles
   - `Toast.Style.Animated`: Loading states
   - `Toast.Style.Success`: Success messages
   - `Toast.Style.Failure`: Error messages

### Debounced Search Pattern
```typescript
useEffect(() => {
  const timer = setTimeout(() => {
    if (searchText) {
      loadData(searchText);
    }
  }, 500);
  return () => clearTimeout(timer);
}, [searchText]);
```
- 500ms delay prevents excessive API calls
- Cleanup function cancels pending timers
- Essential for server-side search

### Multi-Step Flows
- Use state to track current step: `useState<"step1" | "step2">("step1")`
- Conditionally render different views based on step
- Pass data between steps via state
- Example: User selection → Template selection → Action

### Component Reusability
- Export components for use in other commands
- Accept optional props for flexibility (e.g., `userId?: string`)
- Handle both standalone and navigation modes
- Return `null` when no UI needed (e.g., immediate action)

### Error Handling Best Practices
1. Always wrap API calls in try-catch
2. Show user-friendly error messages in toasts
3. Set loading state in finally block
4. Handle specific error cases (e.g., "no active sessions")

### Raycast Extension Structure
```
extensions/raycast/
├── package.json          # Extension manifest
├── tsconfig.json         # TypeScript config
├── README.md             # Documentation
├── assets/               # Icons, screenshots (auto-generated)
├── src/
│   ├── api/
│   │   └── clerk.ts      # API client
│   ├── search-organizations.tsx
│   ├── search-users.tsx
│   ├── impersonate-user.tsx
│   ├── generate-jwt.tsx
│   └── org-members.tsx
└── dist/                 # Build output
```

### Build and Development
- `npm run build`: Production build
- `npm run dev`: Development mode (hot reload)
- `npm run lint`: Check code style
- `npm run fix-lint`: Auto-fix linting issues

### Raycast Assets
- Scaffolding includes default assets (icons, screenshots)
- Replace with custom assets for production
- Icon requirements: 512x512 PNG with transparency
- Screenshots optional but recommended for store listing
