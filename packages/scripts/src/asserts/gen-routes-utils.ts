import { FC, lazy, LazyExoticComponent, ComponentType } from 'react';
type TC = LazyExoticComponent<ComponentType> | FC
export function resolvePagesWithRoutes(names: string[]) {
    return names.reduce((a, b) => {
        const isLazy = b.includes('index.ts')
        const routePath = b.replace(/\/index(.nl)??.tsx/, '')
        const C = isLazy ? lazy(() => import(`#@/pages-with-routes#${b}`)) : require(`#@/pages-with-routes#${b}`).default as FC
        return ({ ...a, [routePath.toLocaleLowerCase()]: C })
    }, {} as { [x: string]: TC })
}