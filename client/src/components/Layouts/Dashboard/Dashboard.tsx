import { PropsWithChildren } from 'preact/compat'

type Props = PropsWithChildren

export const DashboardLayout = ({
	children,
}: Props) => {
	return (
		<div class="flex bg-gray-100 text-gray-900">
			<aside class="flex h-screen w-20 flex-col items-center border-r border-gray-200 bg-white">
				<div class="flex h-[4.5rem] w-full items-center justify-center border-b border-gray-200 p-2">
					<img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcThsapwuIZ2JPUVRaWSoX_xoEIOHWxneY7EupS8gsFriA&s" />
				</div>
				<nav class="flex flex-1 flex-col gap-y-4 pt-10">
					<a href="#" class="group relative rounded-xl bg-gray-100 p-2 text-blue-600 hover:bg-gray-50">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
							<path strokeLinecap="round" strokeLinejoin="round" d="M6 6.878V6a2.25 2.25 0 0 1 2.25-2.25h7.5A2.25 2.25 0 0 1 18 6v.878m-12 0c.235-.083.487-.128.75-.128h10.5c.263 0 .515.045.75.128m-12 0A2.25 2.25 0 0 0 4.5 9v.878m13.5-3A2.25 2.25 0 0 1 19.5 9v.878m0 0a2.246 2.246 0 0 0-.75-.128H5.25c-.263 0-.515.045-.75.128m15 0A2.25 2.25 0 0 1 21 12v6a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 18v-6c0-.98.626-1.813 1.5-2.122" />
						</svg>

						<div class="absolute inset-y-0 left-12 hidden items-center group-hover:flex">
							<div class="relative whitespace-nowrap rounded-md bg-white px-4 py-2 text-sm font-semibold text-gray-900 drop-shadow-lg">
								<div class="absolute inset-0 -left-1 flex items-center">
									<div class="h-2 w-2 rotate-45 bg-white" />
								</div>
									Quizzes
							</div>
						</div>
					</a>
				</nav>

				<div class="flex flex-col items-center gap-y-4 py-10">
					<button class="mt-2 rounded-full bg-gray-100">
						<img class="h-10 w-10 rounded-full" src="https://avatars.githubusercontent.com/u/35387401?v=4" alt="" />
					</button>
				</div>
			</aside>
			<aside class="w-full max-h-screen">
				{children}
			</aside>
		</div>
	)
}
