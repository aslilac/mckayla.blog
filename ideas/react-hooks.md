---
title: You don't understand React hooks.
author: Kayla Washburn
date: 2023.11.13
summary: Thriving in a world without traits or interfaces
tags: programming, react, javascript, hooks
accent_color: "#d178d3"
cover:
  avif: https://cdn.mckayla.cloud/-/ieff75fd8xybx3/uv.avif
  default: https://cdn.mckayla.cloud/-/ieff75fd8xybx3/uv.webp
---

In case you've been living under a rock (or have a life and don't work in web development), React added [hooks] back when they released React 16.8. It's been almost 5 years already, and they've seen incredibly wide adoption in the community. Despite being around for quite a long time[^age] I still see a lot of people using them in ways that I think make their code harder to maintain and understand.

> [!NOTE] I'll show several simplified examples for us to discuss. They're all based on actual production code that I have seen in the wild, all written by competent authors who I think made poor technical choices in these specific cases. We're all human, and make mistakes. I've written bad hooks too!

```ts
const useDocument = (documentId: string) => {
	const userId = useUserId();
	const projectId = useProjectId();
	const data = useData();
	return data.documents[documentId];
};
```

Hooks are also inflexible:

```ts
const data = useProjectData(); // Project | null
// blah blah blah you can't call hooks conditionally
```

Here's how I view hooks: They're for tying React to the outside world. **They are not for tying together things inside React.**

- `useCallback` and `useMemo` preserve object identity as a performance optimization. They boil down to an implementation detail, and honestly, they're primarily for interacting with the _garbage collector_.

- `useState` and `useReducer` provide a way to create reactivity. They're primarily for interacting with the _user_.

- `useEffect` provides a way to connect the React lifecycle to an external _asynchronous system_. (connections, timeouts, non-local event listeners, etc.)

- `useSyncExternalStore` admits it in the name! This hook is for interacting with _mutable data_, through immutable snapshots.

- `useRef` is primarily for interacting with the _DOM_.[^useRef]

- `useContext` is in some ways the only "custom" hook you should ever need. It allows you to create a provider component that interacts with the outside world on your behalf without needing to worry about the details.

This isn't just my opinion though. Even third-party hooks largely stick to this rule.

- `useDebounce` and `useThrottle` let you trigger effects in the outside world with some stricter guarantees about how and when they'll fire. They're for interacting with _expensive effects and compute_.

- `useQuery` is for interacting with _asynchronous data_.

- `useRouter`, `useNavigate`, and `useParams` are for dealing with the fact that we live in, and must interact with, _the browser_.

They're called _hooks_, because they all allow React to _hook into something_.

A "hook" that calls other hooks, but that does not otherwise abstract away the outside world is not a good hook, it's a bad function.

[^age]:

In terms of JavaScript, 5 years is an _actual_ eternity

[^useRef]:

You can also use it to persist values between lifecycles, but I feel that this is a misuse of it.
